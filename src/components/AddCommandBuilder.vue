<template>
  <n-card embedded>
    <n-h3 class="m-0">
      {{ mode === 'edit' ? `Edit Command ${cmd.letter}` : 'New Command Builder' }}
    </n-h3>

    <!-- ───── command meta ───── -->
    <n-form :model="cmd" label-width="120" class="mb-5">
      <n-form-item label="Letter">
        <n-input
            v-model:value="cmd.letter"
            maxlength="1"
            placeholder="A-Z"
            :disabled="letterDisabled"
        />
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="cmd.description" />
      </n-form-item>
    </n-form>

    <!-- ───── add buttons ───── -->
    <n-button-group class="mb-4">
      <n-button @click="addItem('group')">+ Add Group</n-button>
      <n-button @click="addItem('switch')">+ Add Switch</n-button>
    </n-button-group>

    <!-- ───── items list ───── -->
    <div v-for="(item, idx) in cmd.items" :key="idx" class="mb-3 item-block">
      <!-- group -->
      <FieldEditor
          v-if="item.kind === 'group'"
          v-model:fields="item.fields"
          :locked-names="idx === 0 ? ['RSAddress'] : []"
          @remove="idx !== 0 && cmd.items.splice(idx,1)"
      />

      <!-- switch -->
      <SwitchEditor
          v-else
          :item="item"
          :header-fields="headerFields"
          :onRemove="() => cmd.items.splice(idx,1)"
      />
    </div>

    <!-- ───── validation & preview ───── -->
    <n-alert v-if="errors.length" type="error" class="mb-3">
      <ul style="padding-left:20px;margin:0">
        <li v-for="err in errors" :key="err">{{ err }}</li>
      </ul>
    </n-alert>

    <n-text depth="3" class="mb-2">Remaining hex chars: {{ remaining }}</n-text>
    <n-code :code="preview" language="json" class="mb-4" />

    <n-button
        type="primary"
        @click="submit"
        :disabled="errors.length"
    >
      {{ mode === 'edit' ? 'Save' : 'Submit' }}
    </n-button>
    <n-button class="ml-2" v-if="mode==='create'" @click="reset">Reset</n-button>
    <n-button class="ml-2" v-else @click="$emit('cancel')">Cancel</n-button>
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard, NForm, NFormItem, NInput,
  NButton, NButtonGroup, NAlert,
  NText, NCode, useMessage
} from 'naive-ui'
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import FieldEditor  from './FieldEditor.vue'
import SwitchEditor from './SwitchEditor.vue'

/* ---------- props ---------- */
const props = defineProps({
  mode:   { type: String, default: 'create' },        // 'create' | 'edit'
  initial:{ type: Object, default: null },            // command to edit
  existingLetters: { type: Array, default: () => [] } // for create-mode duplicate check
})
const emit = defineEmits(['saved','cancel'])

const deepCopy = (o:any)=> JSON.parse(JSON.stringify(o||{}))

/* ---------- base command template ---------- */
function blankCommand () {
  return {
    letter: '',
    description: '',
    items: [
      {
        kind:'group',
        name:'Header',
        fields:[
          { name:'RSAddress', len:2, base:16, type:'number', description:'Device address on bus' }
        ]
      }
    ]
  }
}

/* ---------- reactive state ---------- */
const cmd = ref(props.mode === 'edit'
    ? deepCopy(props.initial)
    : blankCommand()
)

/* ---------- computed helpers ---------- */
const letterDisabled = computed(()=> props.mode === 'edit')

const headerFields = computed(() =>
    cmd.value.items[0]?.kind==='group'
        ? cmd.value.items[0].fields.map((f:any)=>f.name)
        : []
)

/* ---------- add / remove items ---------- */
function addItem(kind:'group'|'switch'){
  if(kind==='group'){
    cmd.value.items.push({ kind:'group', name:'Group', fields:[] })
  }else{
    cmd.value.items.push({ kind:'switch', switch:'', cases:{}, default:null })
  }
}

/* ---------- validation ---------- */
const LEN_MAP = {2:'u8',4:'u16',6:'u24',8:'u32',16:'u64'} as const
const MAX=21, MIN=18
function allFields(){
  const list:any[]=[]
  cmd.value.items.forEach((it:any)=>{
    if(it.kind==='group') list.push(...it.fields)
    else{
      if(it.cases) Object.values(it.cases).forEach((c:any)=>c.groups.forEach((g:any)=>list.push(...g.fields)))
      if(it.default) it.default.groups.forEach((g:any)=>list.push(...g.fields))
    }
  })
  return list
}
const total     = computed(()=> allFields().reduce((s,f)=>s+f.len,0))
const remaining = computed(()=> MAX-total.value)

const errors = computed(()=>{
  const e:string[]=[]
  /* duplicate letter (create-mode only) */
  if(props.mode==='create' && props.existingLetters.includes(cmd.value.letter)){
    e.push(`Letter ${cmd.value.letter} already exists`)
  }

  function checkGroup(g:any){
    const seen=new Set<string>()
    g.fields.forEach((f:any)=>{
      if(!f.name.trim()) e.push(`Field with no name in "${g.name}"`)
      const k=f.name.trim().toLowerCase()
      if(seen.has(k)) e.push(`Duplicate field "${f.name}" in "${g.name}"`)
      seen.add(k)
      if(f.type==='bool'&&f.len!==2) e.push(`Bool "${f.name}" len≠2`)
      if(f.type==='number'&&!(f.len in LEN_MAP)) e.push(`Len ${f.len} invalid`)
    })
  }
  cmd.value.items.forEach((it:any)=>{
    if(it.kind==='group') checkGroup(it)
    else{
      if(it.cases) Object.values(it.cases).forEach((c:any)=>c.groups.forEach(checkGroup))
      if(it.default) it.default.groups.forEach(checkGroup)
    }
  })
  if(total.value<MIN||total.value>MAX) e.push(`Frame len ${total.value} not ${MIN}-${MAX}`)
  return e
})

/* ---------- preview ---------- */
const preview = computed(()=> JSON.stringify(cmd.value,null,2))

/* ---------- submit ---------- */
const msg = useMessage()
async function submit(){
  if(errors.value.length){ msg.error('Fix validation errors'); return }
  const payload = deepCopy(cmd.value)

  if(props.mode==='edit'){
    await invoke('update_command',{ updatedCmd: payload })
    msg.success('Saved')
  }else{
    await invoke('append_command',{ newCmd: payload })
    msg.success('Created')
  }
  emit('saved', payload)
}

/* ---------- reset (create-mode) ---------- */
function reset(){ cmd.value = blankCommand() }

/* ---------- keep cmd in sync if parent passes new initial (edit switching) ---------- */
watch(()=>props.initial, nv=>{
  if(props.mode==='edit' && nv) cmd.value = deepCopy(nv)
})
</script>

<style scoped>
.item-block{ position:relative; }
</style>
