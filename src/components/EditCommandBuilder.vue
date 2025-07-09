<!-- EditCommandBuilder.vue -->
<template>
  <n-card embedded>
    <n-h3 class="m-0">Edit Command {{ cmd.letter }}</n-h3>

    <!-- ─── meta ─── -->
    <n-form :model="cmd" label-width="120" class="mb-5">
      <n-form-item label="Letter">
        <n-input v-model:value="cmd.letter" disabled />
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="cmd.description" />
      </n-form-item>
    </n-form>

    <!-- ─── add buttons ─── -->
    <n-button-group class="mb-4">
      <n-button @click="addItem('group')">+ Add Group</n-button>
      <n-button @click="addItem('switch')">+ Add Switch</n-button>
    </n-button-group>

    <!-- ─── items ─── -->
    <div v-for="(item, idx) in cmd.items" :key="idx" class="mb-3 item-block">
      <!-- group -->
      <n-card
          v-if="item.kind === 'group'"
          size="small"
          :title="item.name || 'Group'"
          :closable="idx !== 0"
          @close="removeItem(idx)"
      >
        <n-form label-width="110" class="mb-3">
          <n-form-item label="Group name">
            <n-input v-model:value="item.name" :disabled="idx === 0" />
          </n-form-item>
        </n-form>

        <FieldEditor
            v-model:fields="item.fields"
            :locked-names="idx === 0 ? ['RSAddress'] : []"
        />
      </n-card>

      <!-- switch -->
      <SwitchEditor
          v-else
          :item="item"
          :header-fields="headerFields"
          :onRemove="() => removeItem(idx)"
      />
    </div>

    <!-- ─── validation ─── -->
    <n-alert v-if="errors.length" type="error" class="mb-3">
      <ul style="padding-left:20px;margin:0">
        <li v-for="err in errors" :key="err">{{ err }}</li>
      </ul>
    </n-alert>

    <n-code :code="preview" language="json" class="mb-4" />

    <n-button type="primary" @click="submit" :disabled="errors.length > 0">
      Save Changes
    </n-button>
    <n-button class="ml-2" @click="$emit('cancel')">Cancel</n-button>
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard, NForm, NFormItem, NInput, NButton, NButtonGroup,
  NAlert, NCode, useMessage
} from 'naive-ui'
import { ref, computed, watchEffect } from 'vue'
import FieldEditor    from './FieldEditor.vue'
import SwitchEditor   from './SwitchEditor.vue'
import { invoke }     from '@tauri-apps/api/core'

/* ---------- props / emit ---------- */
const props = defineProps<{ command: any }>()
const emit  = defineEmits(['saved','cancel'])

/* ---------- constants ---------- */
const LEN_MAP = { 2:'u8', 4:'u16', 6:'u24', 8:'u32', 16:'u64' } as const
const MAX=21, MIN=18
const msg = useMessage()

/* ---------- helpers ---------- */
function toInternal(items:any[]):any[]{
  return items.map(it=>{
    if ('fields' in it) {            // regular group
      return { kind:'group', ...it }
    }
    // switch
    return { kind:'switch', ...it }
  })
}
function toExternal(items:any[]):any[]{
  return items.map(it=>{
    const copy={...it}; delete copy.kind
    if (it.kind==='group')  return copy
    if (it.kind==='switch') return copy
  })
}

/* ---------- state ---------- */
const cmd = ref<any>({ letter: '', description: '', items: [] })
watchEffect(()=>{
  if (props.command) {
    cmd.value = {
      letter: props.command.letter,
      description: props.command.description,
      items: toInternal(structuredClone(props.command.items))
    }
  }
})

/* ---------- computed ---------- */
const headerFields = computed(()=> cmd.value.items[0]?.fields?.map((f:any)=>f.name) ?? [])

const errors = computed(()=>{
  const e:string[]=[]
  if (!cmd.value) return [];

  // --- Field name and type validation ---
  const allFields: any[] = [];
  cmd.value.items.forEach((it: any) => {
    if (it.kind === 'group') {
      allFields.push(...it.fields);
    } else if (it.kind === 'switch') {
      Object.values(it.cases).forEach((c: any) => c.groups.forEach((g: any) => allFields.push(...g.fields)));
      if (it.default) {
        it.default.groups.forEach((g: any) => allFields.push(...g.fields));
      }
    }
  });

  const seenGlobal = new Set<string>()
  allFields.forEach(f=>{
    if(!f.name?.trim()) e.push('Every field needs a name')
    const id = f.name.toLowerCase()
    if(seenGlobal.has(id)) e.push(`Duplicate field "${f.name}"`)
    seenGlobal.add(id)

    if(f.type==='bool' && f.len!==2) e.push(`Bool "${f.name}" len must be 2`)
    if(f.type==='number' && !(f.len in LEN_MAP)) e.push(`Invalid len for "${f.name}"`)
  })

  // --- New Length Validation Logic ---
  const baseLength = cmd.value.items
      .filter((it: any) => it.kind === 'group')
      .flatMap((it: any) => it.fields)
      .reduce((sum: number, f: any) => sum + f.len, 0);

  const switchItems = cmd.value.items.filter((it: any) => it.kind === 'switch');

  if (switchItems.length === 0) {
    // No switches, just validate the total length
    if (baseLength < MIN || baseLength > MAX) {
      e.push(`Frame length ${baseLength} must be between ${MIN}-${MAX}`);
    }
  } else {
    // Assuming one switch for simplicity.
    const switchItem = switchItems[0];

    // Validate each case
    for (const [key, caseDef] of Object.entries(switchItem.cases as Record<string, any>)) {
      const caseLength = caseDef.groups
          .flatMap((g: any) => g.fields)
          .reduce((sum: number, f: any) => sum + f.len, 0);

      const pathLength = baseLength + caseLength;
      if (pathLength < MIN || pathLength > MAX) {
        e.push(`Path for case '${key}' has invalid length ${pathLength}. Must be between ${MIN}-${MAX}.`);
      }
    }

    // Validate default case
    if (switchItem.default) {
      const defaultLength = switchItem.default.groups
          .flatMap((g: any) => g.fields)
          .reduce((sum: number, f: any) => sum + f.len, 0);

      const pathLength = baseLength + defaultLength;
      if (pathLength < MIN || pathLength > MAX) {
        e.push(`Default path has invalid length ${pathLength}. Must be between ${MIN}-${MAX}.`);
      }
    }
  }

  return e
})

/* ---------- UI actions ---------- */
function addItem(kind:'group'|'switch'){
  if(kind==='group')
    cmd.value.items.push({kind:'group',name:'Group',fields:[]})
  else
    cmd.value.items.push({kind:'switch',switch:'',cases:{},default:null})
}
function removeItem(i:number){ if(i!==0) cmd.value.items.splice(i,1) }

/* ---------- preview ---------- */
const preview = computed(()=> {
  if (!cmd.value) return '';
  return JSON.stringify({
    letter: cmd.value.letter,
    description: cmd.value.description,
    items: toExternal(cmd.value.items).map((it:any)=>{
      if('fields' in it){                        // group
        return {
          ...it,
          fields: it.fields.map((f:any)=>({
            ...f,
            type: f.type==='bool'?'bool':LEN_MAP[f.len as keyof typeof LEN_MAP]
          }))
        }
      }                                          // switch
      const cases:Record<string,any>={}
      for(const [k,v] of Object.entries(it.cases)){
        cases[k.startsWith('0x')?k:`0x${k}`] = v
      }
      const obj:any={ switch:it.switch, cases }
      if(it.default) obj.default = it.default
      return obj
    })
  },null,2)
})

/* ---------- save ---------- */
async function submit(){
  if(errors.value.length){ msg.error('Fix validation errors'); return }
  try{
    await invoke('update_command',{ updatedCmd: JSON.parse(preview.value) })
    msg.success('Command updated')
    emit('saved')
  }catch(e:any){ msg.error(String(e)) }
}
</script>

<style scoped>
.item-block{ position:relative }
</style>
