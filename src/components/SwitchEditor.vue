<template>
  <n-card embedded>
    <n-h3 class="m-0">New Command Builder</n-h3>
    <!-- 1 ── command meta -->
    <n-form :model="cmd" label-width="120" class="mb-5">
      <n-form-item label="Letter">
        <n-input v-model:value="cmd.letter" maxlength="1" placeholder="A-Z" />
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="cmd.description" />
      </n-form-item>
    </n-form>

    <!-- 2 ── add-item buttons -->
    <n-button-group class="mb-4">
      <n-button @click="addItem('group')">+ Add Group</n-button>
      <n-button @click="addItem('switch')">+ Add Switch</n-button>
    </n-button-group>

    <!-- 3 ── items list -->
    <div v-for="(item, idx) in cmd.items" :key="idx" class="mb-3 item-block">
      <!-- GROUP -->
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

        <!-- Header has locked RSAddress -->
        <FieldEditor
            v-model:fields="item.fields"
            :locked-names="idx === 0 ? ['RSAddress'] : []"
        />
      </n-card>

      <!-- SWITCH -->
      <SwitchEditor
          v-else-if="item.kind === 'switch'"
          :item="item"
          :header-fields="headerFields"
          :onRemove="() => removeItem(idx)"
      />
    </div>

    <!-- 4 ── validation & preview -->
    <n-alert v-if="errors.length" type="error" class="mb-3">
      <ul style="padding-left:20px;margin:0">
        <li v-for="err in errors" :key="err">{{ err }}</li>
      </ul>
    </n-alert>

    <n-text depth="3" class="mb-2">
      Remaining hex chars: {{ remaining }}
    </n-text>

    <n-code :code="preview" language="json" class="mb-4" />

    <n-button type="primary" @click="submit" :disabled="submitDisabled">Submit</n-button>
    <n-button class="ml-2" @click="reset">Reset</n-button>

    <!-- reference -->
    <n-divider>Valid data sizes</n-divider>
    <n-table :bordered="false" size="small" style="max-width:520px">
      <thead><tr><th>Len</th><th>Rust type</th></tr></thead>
      <tbody>
      <tr v-for="[l,t] in Object.entries(LEN_MAP)" :key="l">
        <td>{{ l }}</td><td>{{ t }}</td>
      </tr>
      </tbody>
    </n-table>
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard, NForm, NFormItem, NInput, NButton, NButtonGroup,
  NAlert, NText, NDivider, NCode, NTable, useMessage
} from 'naive-ui'
import { ref, computed } from 'vue'
import FieldEditor from './FieldEditor.vue'
import SwitchEditor from './SwitchEditor.vue'
import { invoke } from '@tauri-apps/api/core'

/* ───────── constants ───────── */
const LEN_MAP = { 2:'u8',4:'u16',6:'u24',8:'u32',16:'u64' } as const
const MAX = 21
const MIN = 18
const msg = useMessage()

/* ───────── types ───────── */
interface Field { name:string; len:number; base?:10|16; type:'number'|'bool'; description?:string }
interface Group { kind:'group'; name:string; fields:Field[] }
interface SwitchCase { description?:string; groups:Group[] }
interface SwitchItem { kind:'switch'; switch:string; cases:Record<string,SwitchCase>; default?:SwitchCase|null }
type Item = Group|SwitchItem

/* ───────── initial state ───────── */
const cmd = ref<{ letter:string; description:string; items:Item[] }>({
  letter:'',
  description:'',
  items:[{
    kind:'group', name:'Header',
    fields:[{ name:'RSAddress', len:2, type:'number', base:16, description:'Device address on bus' }]
  }]
})

/* ───────── computed helpers ───────── */
const headerFields = computed(() =>
    cmd.value.items[0].kind==='group' ? cmd.value.items[0].fields.map(f=>f.name) : []
)

/* ───────── add / remove ───────── */
function addItem(kind:'group'|'switch'='group'){
  if(kind==='group') cmd.value.items.push({kind:'group',name:'Group',fields:[]})
  else cmd.value.items.push({kind:'switch',switch:'',cases:{},default:null})
}
function removeItem(i:number){ if(i!==0) cmd.value.items.splice(i,1) }

/* ───────── validation helpers ───────── */
function allFields():Field[]{
  const list:Field[]=[]
  cmd.value.items.forEach(it=>{
    if(it.kind==='group') list.push(...it.fields)
    else {
      Object.values(it.cases).forEach(c=>c.groups.forEach(g=>list.push(...g.fields)))
      if(it.default) it.default.groups.forEach(g=>list.push(...g.fields))
    }
  })
  return list
}
const total = computed(()=> allFields().reduce((s,f)=>s+f.len,0))
const remaining = computed(()=> MAX-total.value)
const errors = computed(()=>{
  const e:string[]=[]
  for(const f of allFields()){
    if(f.type==='bool' && f.len!==2) e.push(`Bool "${f.name}" len must be 2`)
    if(f.type==='number' && !(f.len in LEN_MAP)) e.push(`Field "${f.name}" len ${f.len} not 2/4/6/8/16`)
  }
  if(total.value<MIN || total.value>MAX) e.push(`Frame length ${total.value} must be ${MIN}-${MAX}`)
  return e
})
const submitDisabled = computed(()=> errors.value.length>0)

/* ───────── util: prefix 0x for hex keys ───────── */
const hexKey = (k:string)=> k.startsWith('0x') ? k : `0x${k}`

/* ───────── preview JSON ───────── */
const preview = computed(()=> JSON.stringify({
  letter: cmd.value.letter.toUpperCase(),
  description: cmd.value.description,
  items: cmd.value.items.map(it=>{
    if(it.kind==='group'){
      return {
        name: it.name,
        fields: it.fields.map(f=>({
          name:f.name, len:f.len,
          base:f.type==='bool'?undefined:f.base,
          type:f.type==='bool'?'bool':LEN_MAP[f.len as keyof typeof LEN_MAP],
          description:f.description
        }))
      }
    }
    /* switch */
    const sw = it as SwitchItem
    const cases:Record<string,any> = {}
    for(const [k,v] of Object.entries(sw.cases)){
      cases[hexKey(k)] = {
        description:v.description,
        groups:v.groups.map(g=>({
          name:g.name,
          fields:g.fields.map(f=>({
            name:f.name, len:f.len,
            base:f.type==='bool'?undefined:f.base,
            type:f.type==='bool'?'bool':LEN_MAP[f.len as keyof typeof LEN_MAP],
            description:f.description
          }))
        }))
      }
    }
    const obj:any = { switch: sw.switch, cases }
    if(sw.default){
      obj.default = {
        description: sw.default.description,
        groups: sw.default.groups.map(g=>({
          name:g.name,
          fields:g.fields.map(f=>({
            name:f.name, len:f.len,
            base:f.type==='bool'?undefined:f.base,
            type:f.type==='bool'?'bool':LEN_MAP[f.len as keyof typeof LEN_MAP],
            description:f.description
          }))
        }))
      }
    }
    return obj
  })
}, null, 2))

/* ───────── actions ───────── */
async function submit(){
  if(submitDisabled.value){ msg.error('Fix errors first'); return }
  try{
    await invoke('append_command',{ newCmd: JSON.parse(preview.value) })
    msg.success('Saved!')
  }catch(e:any){ msg.error(String(e)) }
}
function reset(){
  cmd.value.letter=''
  cmd.value.description=''
  cmd.value.items=[{
    kind:'group', name:'Header',
    fields:[{ name:'RSAddress', len:2, type:'number', base:16, description:'Device address on bus' }]
  }]
}
</script>

<style scoped>
.item-block{ position:relative; }
</style>
