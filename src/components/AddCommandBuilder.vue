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

    <n-text depth="3" class="mb-2">Total field length: {{ total }} / {{ minFieldLength }}-{{ maxFieldLength }} hex chars</n-text>
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
            type: 'number', // CORRECTED: Use 'type' for UI binding
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
            type: 'number', // CORRECTED: Use 'type' for UI binding
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
      fields: [{name: 'field-1', len: 2, base: 16, type: 'number', description: ''}] // CORRECTED: Use 'type'
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
            fields: [{name: 'field-1', len: 2, base: 16, type: 'number', description: ''}] // CORRECTED: Use 'type'
          }]
        }
      },
      default: null
    })
  }
}

/* ---------- validation ---------- */
const minFieldLength = computed(() => {
  if (!props.family) return 0;
  const commandLetterLength = 1;
  const maxTerminatorLength = 3;
  return props.family.frame_len - props.family.start.length - maxTerminatorLength - commandLetterLength;
});

const maxFieldLength = computed(() => {
  if (!props.family) return 0;
  const commandLetterLength = 1;
  const minTerminatorLength = 1;
  return props.family.frame_len - props.family.start.length - minTerminatorLength - commandLetterLength;
});


function effectiveFields(): any[] {
  const out: any[] = []
  cmd.value.items.forEach((it: any) => {
    if (it.kind === 'group') out.push(...it.fields)
    else {
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

  if (props.mode === 'create' && props.existingLetters.includes(cmd.value.letter)) {
    e.push(`Letter ${cmd.value.letter} already exists in this family.`)
  }

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

  if (total.value < minFieldLength.value || total.value > maxFieldLength.value) {
    e.push(`Total field length must be between ${minFieldLength.value} and ${maxFieldLength.value} characters.`)
  }

  return e
})

/* ---------- preview & submission payload ---------- */
function getTransformedItems() {
  // Create a deep copy to avoid mutating the reactive UI state `cmd`
  const itemsCopy = deepCopy(cmd.value.items);

  const lenToTypeMap: { [key: number]: string } = {
    2: 'u8',
    4: 'u16',
    6: 'u24',
    8: 'u32',
    16: 'u64'
  };

  const transformField = (field: any) => {
    if (field.type === 'number') {
      // Convert UI-friendly 'number' type to specific backend type (e.g., 'u8')
      field.type = lenToTypeMap[field.len] || 'u32';
    }
    // 'bool' type is already valid for the backend, so no change is needed
  };

  itemsCopy.forEach((item: any) => {
    if (item.kind === 'group') {
      item.fields.forEach(transformField);
    } else if (item.kind === 'switch') {
      Object.values(item.cases).forEach((c: any) => {
        c.groups.forEach((g: any) => g.fields.forEach(transformField));
      });
      if (item.default) {
        item.default.groups.forEach((g: any) => g.fields.forEach(transformField));
      }
    }
  });

  return stripKind(itemsCopy);
}

const preview = computed(() => JSON.stringify({
  letter: cmd.value.letter,
  description: cmd.value.description,
  items: getTransformedItems()
}, null, 2))


/* ---------- submit / reset ---------- */
async function submit() {
  if (errors.value.length) {
    msg.error('Please fix validation errors before submitting.');
    return
  }
  // The 'preview' computed property now generates the correct backend payload
  const payload = JSON.parse(preview.value)
  try {
    if (props.mode === 'edit') {
      await invoke('update_command', {
        familyStart: props.family.start,
        originalLetter: props.initial.letter,
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

/* CORRECTED: Widen the number input for length */
.item-block :deep(.field-len) {
  width: 90px;
}
</style>
