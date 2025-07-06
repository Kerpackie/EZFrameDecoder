<template>
  <n-card embedded>
    <n-h3 class="m-0">
      {{ mode === 'edit' ? `Edit Command ${cmd.letter}` : 'New Command Builder' }}
    </n-h3>

    <!-- 1 ── meta -->
    <n-form :model="cmd" label-width="120" class="mb-5">
      <n-form-item label="Letter">
        <n-input
            v-model:value="cmd.letter"
            maxlength="1"
            placeholder="A-Z"
            :disabled="mode === 'edit'"
        />
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="cmd.description" />
      </n-form-item>
    </n-form>

    <!-- 2 ── add buttons -->
    <n-button-group class="mb-4">
      <n-button @click="addItem('group')">+ Add Group</n-button>
      <n-button @click="addItem('switch')">+ Add Switch</n-button>
    </n-button-group>

    <!-- 3 ── items -->
    <div v-for="(item, idx) in cmd.items" :key="idx" class="mb-3 item-block">
      <!-- GROUP -->
      <n-card
          v-if="item.kind === 'group'"
          size="small"
          :title="item.name || 'Group'"
          :closable="idx !== 0"
          @close="idx !== 0 && cmd.items.splice(idx, 1)"
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

      <!-- SWITCH -->
      <SwitchEditor
          v-else
          :item="item"
          :header-fields="headerFields"
          :onRemove="() => cmd.items.splice(idx, 1)"
      />
    </div>

    <!-- 4 ── validation & preview -->
    <n-alert v-if="errors.length" type="error" class="mb-3">
      <ul style="padding-left:20px;margin:0">
        <li v-for="err in errors" :key="err">{{ err }}</li>
      </ul>
    </n-alert>

    <n-text depth="3" class="mb-2">Remaining hex chars: {{ remaining }}</n-text>
    <n-code :code="preview" language="json" class="mb-4" />

    <n-button type="primary" @click="submit" :disabled="errors.length">
      {{ mode === 'edit' ? 'Save' : 'Submit' }}
    </n-button>
    <n-button v-if="mode==='create'" class="ml-2" @click="reset">Reset</n-button>
    <n-button v-else class="ml-2" @click="$emit('cancel')">Cancel</n-button>
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
  mode:            { type: String, default: 'create' }, // create | edit
  initial:         { type: Object, default: null },
  existingLetters: { type: Array,  default: () => [] }
})
const emit = defineEmits(['saved','cancel'])
const msg  = useMessage()

/* ---------- util ---------- */
const deepCopy = (o:any)=> JSON.parse(JSON.stringify(o ?? {}))

/* --------- new helper: convert rust-type → abstract type --------- */
function normaliseField (f:any) {
  if (typeof f.type === 'string' && f.type.startsWith('u')) {
    const lenMap = { u8:2, u16:4, u24:6, u32:8, u64:16 }
    const len = lenMap[f.type as keyof typeof lenMap]
    if (len) {
      f.type = 'number'
      f.len  = len
    }
  }
}

/* ---------- normalise items (kind + field conversion) ---------- */
function normalise (items:any[]) {
  items.forEach((it:any)=>{
    if ('fields' in it) {
      it.kind = 'group'
      if (!it.name) it.name = 'Group'
      it.fields.forEach(normaliseField)
    } else {
      it.kind = 'switch'
      Object.values(it.cases).forEach((c:any)=>
          c.groups.forEach((g:any)=> g.fields.forEach(normaliseField))
      )
      it.default?.groups.forEach((g:any)=>
          g.fields.forEach(normaliseField)
      )
    }
  })
}

/* ---------- strip helper ---------- */
function stripKind(items:any[]){ return items.map(({kind, ...r}:any)=>r) }

/* ---------- blank template ---------- */
function blankCommand(){
  return {
    letter:'', description:'',
    items:[{
      kind:'group', name:'Header',
      fields:[{ name:'RSAddress', len:2, base:16, type:'number', description:'Device address on bus' }]
    }]
  }
}

/* ---------- state ---------- */
const cmd = ref(
    props.mode==='edit'
        ? (() => {
          const c = deepCopy(props.initial)
          normalise(c.items)     // <-- convert u8/u16 → number+len
          return c
        })()
        : blankCommand()
)

/* ---------- header field list ---------- */
const headerFields = computed(()=> cmd.value.items[0]?.fields?.map((f:any)=>f.name) ?? [])

/* ---------- add items (unchanged) ---------- */
function addItem(kind:'group'|'switch'){
  if (kind==='group'){
    cmd.value.items.push({
      kind:'group', name:'Group',
      fields:[{ name:'field-1', len:2, base:16, type:'number', description:'' }]
    })
  } else {
    const header = headerFields.value.find(n=>n!=='RSAddress') || headerFields.value[0] || ''
    cmd.value.items.push({
      kind:'switch',
      switch: header,
      cases:{
        '0x0000':{
          description:'',
          groups:[{
            name:'Group',
            fields:[{ name:'field-1', len:2, base:16, type:'number', description:'' }]
          }]
        }
      },
      default:null
    })
  }
}

/* ---------- validation (unchanged except bool len=1 rule kept) ---------- */
const LEN_MAP = {2:'u8',4:'u16',6:'u24',8:'u32',16:'u64'} as const
const MAX=21, MIN=18

function effectiveFields():any[]{
  const out:any[]=[]
  cmd.value.items.forEach((it:any)=>{
    if(it.kind==='group') out.push(...it.fields)
    else {
      const firstCase:any = Object.values(it.cases)[0]
      const variant:any   = firstCase ?? it.default
      variant && variant.groups.forEach((g:any)=> out.push(...g.fields))
    }
  })
  return out
}

const total     = computed(()=> effectiveFields().reduce((s,f)=>s+f.len,0))
const remaining = computed(()=> MAX-total.value)

const errors = computed(()=>{
  const e:string[]=[]
  if (props.mode==='create' && props.existingLetters.includes(cmd.value.letter))
    e.push(`Letter ${cmd.value.letter} already exists`)

  /* dedupe & rules */
  cmd.value.items.forEach((it:any)=>{
    const groups = it.kind==='group'
        ? [it]
        : [
          ...Object.values(it.cases).flatMap((v:any)=>v.groups),
          ...(it.default?.groups ?? [])
        ]
    groups.forEach((g:any)=>{
      const seen=new Set<string>()
      g.fields.forEach((f:any)=>{
        const id=f.name.trim().toLowerCase()
        if (!id) e.push('Unnamed field')
        if (seen.has(id)) e.push(`Duplicate field "${f.name}" in "${g.name}"`)
        seen.add(id)
        if (f.type==='bool'   && f.len!==1)             e.push(`Bool "${f.name}" len≠1`)
        if (f.type==='number' && !(f.len in LEN_MAP))   e.push(`Len ${f.len} invalid`)
      })
    })
  })

  if (total.value<MIN || total.value>MAX)
    e.push(`Frame len ${total.value} not ${MIN}-${MAX}`)
  return e
})

/* ---------- preview ---------- */
const preview = computed(()=> JSON.stringify({
  letter: cmd.value.letter,
  description: cmd.value.description,
  items: stripKind(cmd.value.items)
}, null, 2))

/* ---------- submit / reset (unchanged) ---------- */
async function submit(){
  if(errors.value.length){ msg.error('Fix validation errors'); return }
  const payload = JSON.parse(preview.value)
  if(props.mode==='edit'){
    await invoke('update_command',{ updatedCmd: payload })
    msg.success('Saved')
  }else{
    await invoke('append_command',{ newCmd: payload })
    msg.success('Created')
  }
  emit('saved', payload)
}
function reset(){ cmd.value = blankCommand() }

/* ---------- watch for prop updates ---------- */
watch(()=>props.initial, nv=>{
  if(props.mode==='edit' && nv){
    const c = deepCopy(nv)
    normalise(c.items)          // <-- ensure fresh load is normalised
    cmd.value = c
  }
})
</script>

<style scoped>
.item-block{ position:relative; }
</style>
