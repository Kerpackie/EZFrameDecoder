<template>
  <n-card embedded>
    <n-h3 class="m-0">New Command Builder</n-h3>

    <n-form :model="command" label-width="120" class="mb-4">
      <n-form-item label="Letter">
        <n-input v-model:value="command.letter" maxlength="1" placeholder="A‑Z"/>
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="command.description"/>
      </n-form-item>
    </n-form>

    <n-text depth="3" class="mb-2">
      Remaining hex chars: {{ remainingLen }} / {{ MAX_DATA_LEN }}
    </n-text>

    <n-divider title-placement="left">Groups</n-divider>

    <div v-for="(group, gIdx) in command.items" :key="gIdx" class="group-block mb-4">
      <n-card
          size="small"
          :title="group.name || 'Group'"
          :closable="gIdx !== 0"
          @close="removeGroup(gIdx)"
      >
        <!-- group name -->
        <n-form label-width="120" class="mb-3">
          <n-form-item label="Group Name">
            <n-input v-model:value="group.name" :disabled="gIdx === 0"/>
          </n-form-item>
        </n-form>

        <!-- fields -->
        <n-h5 class="mt-0">Fields</n-h5>

        <div v-for="(field, fIdx) in group.fields" :key="fIdx" class="field-block mb-2">
          <n-space align="center" wrap>
            <n-input v-model:value="field.name" placeholder="Name" style="width: 120px"/>
            <n-input-number v-model:value="field.len" :min="1" :max="16" placeholder="Len" style="width: 70px"/>
            <n-select v-model:value="field.kind" :options="kindOptions" style="width: 90px"/>
            <n-select v-if="field.kind === 'number'" v-model:value="field.base" :options="baseOptions"
                      style="width: 65px"/>
            <n-input v-model:value="field.description" placeholder="Description" style="width: 150px"/>
            <n-button size="tiny" type="error" @click="removeField(gIdx, fIdx)" :disabled="gIdx === 0 && fIdx === 0">✕
            </n-button>
          </n-space>
        </div>

        <n-button tertiary size="tiny" @click="addField(gIdx)" :disabled="remainingLen <= 0">+ Add Field</n-button>
      </n-card>
    </div>

    <n-button type="primary" @click="addGroup" class="mb-4" :disabled="remainingLen <= 0">+ Add Group</n-button>

    <n-alert v-if="errors.length" type="error" :show-icon="false" class="mb-3">
      <div v-for="e in errors" :key="e">{{ e }}</div>
    </n-alert>

    <n-divider title-placement="left">Preview</n-divider>
    <n-code :code="previewJson" language="json" class="mb-4"/>

    <n-space class="mb-6">
      <n-button type="primary" @click="submit" :disabled="submitDisabled">Submit</n-button>
      <n-button @click="reset">Reset</n-button>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard, NH3, NForm, NFormItem, NInput, NInputNumber, NButton,
  NSpace, NDivider, NCode, NSelect, NText, NAlert, NH5, useMessage
} from "naive-ui";
import {ref, computed} from "vue";
import {invoke} from "@tauri-apps/api/core";

const message = useMessage();

/* frame constants */
const MIN_DATA_LEN = 18; // for >>> terminator
const MAX_DATA_LEN = 20; // for > terminator

/* mapping len→ concrete unsigned type */
function concreteType(len: number): string {
  if (len <= 2) return "u8";
  if (len <= 4) return "u16";
  if (len <= 6) return "u24";
  if (len <= 8) return "u32";
  return "u64"; // up to 16
}

interface FieldDraft {
  name: string;
  len: number;
  base?: 10 | 16;
  kind: "number" | "bool"; // UI-level type
  description?: string;
}

interface GroupDraft {
  name: string;
  fields: FieldDraft[];
}

const command = ref<{ letter: string; description: string; items: GroupDraft[] }>({
  letter: "",
  description: "",
  items: [
    {
      name: "Header",
      fields: [
        {name: "RSAddress", len: 2, base: 16, kind: "number", description: "Device address"}
      ]
    }
  ]
});

/* select options */
const kindOptions = [
  {label: "number", value: "number"},
  {label: "bool", value: "bool"}
];
const baseOptions = [{label: "16", value: 16}, {label: "10", value: 10}];

/* helpers */
function addGroup() {
  command.value.items.push({name: "", fields: []});
}

function removeGroup(i: number) {
  if (i === 0) return;
  command.value.items.splice(i, 1);
}

function addField(g: number) {
  command.value.items[g].fields.push({name: "", len: 2, base: 16, kind: "number", description: ""});
}

function removeField(g: number, f: number) {
  if (g === 0 && f === 0) return;
  command.value.items[g].fields.splice(f, 1);
}

/* totals */
const totalLen = computed(() =>
    command.value.items.flatMap(g => g.fields).reduce((s, f) => s + f.len, 0)
);
const remainingLen = computed(() => MAX_DATA_LEN - totalLen.value);

/* validation */
const errors = computed(() => {
  const list: string[] = [];
  // basic frame length bounds
  if (totalLen.value > MAX_DATA_LEN) list.push(`Total ${totalLen.value}/${MAX_DATA_LEN} exceeds frame limit`);
  if (totalLen.value < MIN_DATA_LEN) list.push(`Total ${totalLen.value}/${MAX_DATA_LEN} – needs at least ${MIN_DATA_LEN}`);
  // field sanity
  command.value.items.forEach((g, gi) =>
      g.fields.forEach((f, fi) => {
        if (f.kind === "bool" && f.len !== 2) {
          list.push(`Group ${gi + 1} • Field ${fi + 1} \"${f.name}\" bool must be len 2`);
        }
        if (f.len > 16) list.push(`Field \"${f.name}\" len >16 not allowed`);
      })
  );
  return list;
});

const submitDisabled = computed(() => errors.value.length > 0);

/* preview build */
function buildConcrete() {
  return command.value.items.map(g => ({
    name: g.name,
    fields: g.fields.map(f => ({
      name: f.name,
      len: f.len,
      ...(f.kind === "bool"
          ? {type: "bool"}
          : {type: concreteType(f.len), base: f.base}),
      description: f.description || undefined
    }))
  }));
}

const previewJson = computed(() =>
    JSON.stringify(
        {
          letter: command.value.letter.toUpperCase(),
          description: command.value.description,
          items: buildConcrete()
        },
        null,
        2
    )
);

function reset() {
  command.value = {
    letter: "",
    description: "",
    items: [
      {
        name: "Header",
        fields: [
          {name: "RSAddress", len: 2, base: 16, kind: "number", description: "Device address"}
        ]
      }
    ]
  };
}

async function submit() {
  if (submitDisabled.value) return;
  try {
    await invoke("append_command", {
      newCmd: {
        letter: command.value.letter.toUpperCase(),
        description: command.value.description,
        items: buildConcrete()
      }
    });
    message.success("Command saved!");
    reset();
  } catch (e: any) {
    message.error(String(e));
  }
}
</script>

<style scoped>
.group-block {
  border-left: 4px solid #eee;
  padding-left: 1rem;
}

.field-block {
  margin-left: 1rem;
}
</style>
