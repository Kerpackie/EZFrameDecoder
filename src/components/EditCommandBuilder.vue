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

    <n-text depth="3" class="mb-2">Remaining hex chars: {{ remaining }}</n-text>
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
  NAlert, NText, NCode, useMessage
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
const cmd = ref<any>(null)
watchEffect(()=>{
  cmd.value = {
    letter: props.command.letter,
    description: props.command.description,
    items: toInternal(structuredClone(props.command.items))
  }
})

/* ---------- computed ---------- */
const headerFields = computed(()=> cmd.value.items[0]?.fields?.map((f:any)=>f.name) ?? [])

function collectFields(){
  const out:any[]=[]
  cmd.value.items.forEach((it:any)=>{
    if(it.kind==='group') out.push(...it.fields)
    else{
      Object.values(it.cases).forEach((c:any)=>c.groups.forEach((g:any)=>out.push(...g.fields)))
      it.default && it.default.groups.forEach((g:any)=>out.push(...g.fields))
    }
  })
  return out
}
const total      = computed(()=> collectFields().reduce((s,f)=>s+f.len,0))
const remaining  = computed(()=> MAX-total.value)
const errors     = computed(()=>{
  const e:string[]=[]
  const seenGlobal = new Set<string>()
  collectFields().forEach(f=>{
    if(!f.name?.trim()) e.push('Every field needs a name')
    const id = f.name.toLowerCase()
    if(seenGlobal.has(id)) e.push(`Duplicate field "${f.name}"`)
    seenGlobal.add(id)

    if(f.type==='bool' && f.len!==2)                e.push(`Bool "${f.name}" len must be 2`)
    if(f.type==='number' && !(f.len in LEN_MAP))    e.push(`Invalid len for "${f.name}"`)
  })
  if(total.value<MIN||total.value>MAX)
    e.push(`Frame length ${total.value} must be between ${MIN}-${MAX}`)
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
const preview = computed(()=> JSON.stringify({
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
},null,2))

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
