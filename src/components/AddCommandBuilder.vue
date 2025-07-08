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
        <n-input v-model:value="cmd.description"/>
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
            <n-input v-model:value="item.name" :disabled="idx === 0"/>
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

    <n-text depth="3" class="mb-2">Total field length: {{ total }} / {{ expectedFieldLength }} hex chars</n-text>
    <n-code :code="preview" language="json" class="mb-4"/>

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
import {ref, computed, watch} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import FieldEditor from './FieldEditor.vue'
import SwitchEditor from './SwitchEditor.vue'

/* ---------- props ---------- */
const props = defineProps({
  mode: {type: String, default: 'create'}, // create | edit
  initial: {type: Object, default: null},
  family: { type: Object, required: true }, // The entire family object is needed for validation
  existingLetters: {type: Array, default: () => []}
})
const emit = defineEmits(['saved', 'cancel'])
const msg = useMessage()

/* ---------- util ---------- */
const deepCopy = (o: any) => JSON.parse(JSON.stringify(o ?? {}))

// This function is no longer needed as the backend handles type coercion.
// We keep the structure for potential future frontend-only logic.
function normaliseField(f: any) {
  // No-op, backend handles types.
}

/* ---------- normalise items (kind + field conversion) ---------- */
function normalise(items: any[]) {
  items.forEach((it: any) => {
    if ('fields' in it) {
      it.kind = 'group'
      if (!it.name) it.name = 'Group'
      it.fields.forEach(normaliseField)
    } else {
      it.kind = 'switch'
      Object.values(it.cases).forEach((c: any) =>
          c.groups.forEach((g: any) => g.fields.forEach(normaliseField))
      )
      it.default?.groups.forEach((g: any) =>
          g.fields.forEach(normaliseField)
      )
    }
  })
}

/* ---------- strip helper ---------- */
function stripKind(items: any[]) {
  return items.map(({kind, ...r}: any) => r)
}

/* ---------- blank template ---------- */
function blankCommand() {
  return {
    letter: '',
    description: '',
    items: [
      {
        kind: 'group',
        name: 'Header',
        fields: [
          {
            name: 'RSAddress',
            len: 2,
            base: 16,
            typ: 'u8', // Use backend types
            description: 'Device address on bus'
          }
        ]
      },
      {
        kind: 'group',
        name: 'Payload',
        fields: [
          {
            name: 'field-1',
            len: 2,
            base: 16,
            typ: 'u8', // Use backend types
            description: ''
          }
        ]
      }
    ]
  }
}

/* ---------- state ---------- */
const cmd = ref(
    props.mode === 'edit'
        ? (() => {
          const c = deepCopy(props.initial)
          normalise(c.items)
          return c
        })()
        : blankCommand()
)

/* ---------- header field list ---------- */
const headerFields = computed(() => cmd.value.items[0]?.fields?.map((f: any) => f.name) ?? [])

/* ---------- add items ---------- */
function addItem(kind: 'group' | 'switch') {
  if (kind === 'group') {
    cmd.value.items.push({
      kind: 'group', name: 'Group',
      fields: [{name: 'field-1', len: 2, base: 16, typ: 'u8', description: ''}]
    })
  } else {
    const header = headerFields.value.find(n => n !== 'RSAddress') || headerFields.value[0] || ''
    cmd.value.items.push({
      kind: 'switch',
      switch: header,
      cases: {
        '0x0000': {
          description: '',
          groups: [{
            name: 'Group',
            fields: [{name: 'field-1', len: 2, base: 16, typ: 'u8', description: ''}]
          }]
        }
      },
      default: null
    })
  }
}

/* ---------- validation ---------- */
const expectedFieldLength = computed(() => {
  // Total frame length minus start, terminator, and the command letter itself.
  if (!props.family) return 0;
  const commandLetterLength = 1;
  return props.family.frame_len - props.family.start.length - props.family.terminator.length - commandLetterLength;
});


function effectiveFields(): any[] {
  const out: any[] = []
  cmd.value.items.forEach((it: any) => {
    if (it.kind === 'group') out.push(...it.fields)
    else {
      // For validation, we can check the length of the first variant's fields as a representative.
      // A more complex validation could ensure all variants have the same length.
      const firstCase: any = Object.values(it.cases)[0]
      const variant: any = firstCase ?? it.default
      variant && variant.groups.forEach((g: any) => out.push(...g.fields))
    }
  })
  return out
}

const total = computed(() => effectiveFields().reduce((s, f) => s + f.len, 0))

const errors = computed(() => {
  const e: string[] = []

  /* duplicate letter (create-mode only) */
  if (
      props.mode === 'create' &&
      props.existingLetters.includes(cmd.value.letter)
  ) {
    e.push(`Letter ${cmd.value.letter} already exists in this family.`)
  }

  /* per-group duplicate & per-field rules */
  cmd.value.items.forEach((it: any) => {
    const groups =
        it.kind === 'group'
            ? [it]
            : [
              ...Object.values(it.cases).flatMap((v: any) => v.groups),
              ...(it.default?.groups ?? [])
            ]

    groups.forEach((g: any) => {
      const seen = new Set<string>()
      g.fields.forEach((f: any) => {
        const id = f.name.trim().toLowerCase()
        if (!id) e.push('A field name cannot be empty.')
        if (seen.has(id))
          e.push(`Duplicate field "${f.name}" in group "${g.name}".`)
        seen.add(id)
      })
    })
  })

  /* effective-length rule */
  if (total.value !== expectedFieldLength.value)
    e.push(`Total length of all fields must be exactly ${expectedFieldLength.value} characters.`)

  return e
})

/* ---------- preview ---------- */
const preview = computed(() => JSON.stringify({
  letter: cmd.value.letter,
  description: cmd.value.description,
  items: stripKind(cmd.value.items)
}, null, 2))

/* ---------- submit / reset ---------- */
async function submit() {
  if (errors.value.length) {
    msg.error('Please fix validation errors before submitting.');
    return
  }
  const payload = JSON.parse(preview.value)
  try {
    if (props.mode === 'edit') {
      await invoke('update_command', {
        familyStart: props.family.start,
        originalLetter: props.initial.letter, // Pass the original letter for lookup
        cmd: payload
      })
      msg.success(`Command '${payload.letter}' saved successfully.`)
    } else {
      await invoke('append_command', {
        familyStart: props.family.start,
        cmd: payload
      })
      msg.success(`Command '${payload.letter}' created successfully.`)
    }
    emit('saved', payload)
  } catch (err) {
    msg.error(String(err));
    console.error(err);
  }
}

function reset() {
  cmd.value = blankCommand()
}

/* ---------- watch for prop updates ---------- */
watch(() => props.initial, nv => {
  if (props.mode === 'edit' && nv) {
    const c = deepCopy(nv)
    normalise(c.items)
    cmd.value = c
  }
})
</script>

<style scoped>
.item-block {
  position: relative;
}
</style>
