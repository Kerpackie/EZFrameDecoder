<template>
  <n-card embedded>
    <n-h3 class="m-0">New Command Builder</n-h3>

    <!-- 1. Command Metadata -->
    <n-form :model="command" label-width="120" class="mb-6">
      <n-form-item label="Letter">
        <n-input v-model:value="command.letter" maxlength="1" placeholder="A-Z" />
      </n-form-item>
      <n-form-item label="Description">
        <n-input v-model:value="command.description" />
      </n-form-item>
    </n-form>

    <!-- 2. Groups -->
    <n-divider title-placement="left">Groups</n-divider>

    <div v-for="(group, gIdx) in command.items" :key="gIdx" class="group-block mb-4">
      <n-card
          size="small"
          :title="group.name || 'Group'"
          :closable="gIdx !== 0"
          @close="removeGroup(gIdx)"
      >
        <n-form label-width="120" class="mb-3">
          <n-form-item label="Group Name">
            <n-input v-model:value="group.name" :disabled="gIdx === 0" />
          </n-form-item>
        </n-form>

        <n-h5 class="mt-0">Fields</n-h5>

        <div v-for="(field, fIdx) in group.fields" :key="fIdx" class="field-block mb-2">
          <n-space align="center" wrap>
            <n-input v-model:value="field.name" placeholder="Name" style="width: 130px" />
            <n-input-number v-model:value="field.len" :min="1" placeholder="Len" style="width: 80px" />
            <n-select v-model:value="field.type" :options="typeOptions" style="width: 90px" />
            <n-select
                v-if="field.type !== 'bool'"
                v-model:value="field.base"
                :options="baseOptions"
                style="width: 70px"
            />
            <n-input v-model:value="field.description" placeholder="Description" style="width: 160px" />
            <n-button
                size="tiny"
                type="error"
                @click="removeField(gIdx, fIdx)"
                :disabled="gIdx === 0 && fIdx === 0"
            >
              ✕
            </n-button>
          </n-space>
        </div>

        <n-button tertiary size="tiny" @click="addField(gIdx)">+ Add Field</n-button>
      </n-card>
    </div>

    <n-button type="primary" @click="addGroup" class="mb-4">+ Add Group</n-button>

    <!-- 3. Preview -->
    <n-divider title-placement="left">Preview</n-divider>
    <n-code :code="previewJson" language="json" class="mb-4" />

    <!-- 4. Actions -->
    <n-space class="mb-6">
      <n-button type="primary" @click="submit">Submit</n-button>
      <n-button @click="reset">Reset</n-button>
    </n-space>

    <!-- 5. Field Type Table -->
    <n-divider>Valid Field Types</n-divider>
    <n-text depth="3" class="mb-2">
      <code>len</code> counts hex characters (2 chars = 1 byte).
    </n-text>

    <n-table :bordered="false" size="small" style="max-width: 520px;">
      <thead>
      <tr><th>Len</th><th>Bytes</th><th>Type</th></tr>
      </thead>
      <tbody>
      <tr><td>2</td><td>1</td><td><code>u8</code></td></tr>
      <tr><td>4</td><td>2</td><td><code>u16</code></td></tr>
      <tr><td>6</td><td>3</td><td><code>u24</code></td></tr>
      <tr><td>8</td><td>4</td><td><code>u32</code></td></tr>
      <tr><td>16</td><td>8</td><td><code>u64</code></td></tr>
      <tr><td>2</td><td>1</td><td><code>bool</code></td></tr>
      </tbody>
    </n-table>
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard, NH3, NForm, NFormItem, NInput, NInputNumber, NButton,
  NSpace, NDivider, NCode, NSelect, NText, NTable, NH5, useMessage
} from "naive-ui";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const message = useMessage();

interface Field {
  name: string;
  len: number;
  base?: 10 | 16;
  type: string;
  description?: string;
}
interface Group {
  name: string;
  fields: Field[];
}

const command = ref({
  letter: "",
  description: "",
  items: [
    {
      name: "Header",
      fields: [
        {
          name: "RSAddress",
          len: 2,
          base: 16,
          type: "u8",
          description: "Device address"
        }
      ]
    }
  ] as Group[]
});

const typeOptions = ["u8", "u16", "u24", "u32", "u64", "bool"].map(t => ({ label: t, value: t }));
const baseOptions = [
  { label: "16", value: 16 },
  { label: "10", value: 10 }
];

function addGroup() {
  command.value.items.push({ name: "", fields: [] });
}
function removeGroup(idx: number) {
  if (idx === 0) return;
  command.value.items.splice(idx, 1);
}
function addField(gIdx: number) {
  command.value.items[gIdx].fields.push({
    name: "",
    len: 1,
    base: 16,
    type: "u8",
    description: ""
  });
}
function removeField(gIdx: number, fIdx: number) {
  if (gIdx === 0 && fIdx === 0) return;
  command.value.items[gIdx].fields.splice(fIdx, 1);
}

const previewJson = computed(() =>
    JSON.stringify(
        {
          letter: command.value.letter.toUpperCase(),
          description: command.value.description,
          items: command.value.items
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
          {
            name: "RSAddress",
            len: 2,
            base: 16,
            type: "u8",
            description: "Device address"
          }
        ]
      }
    ]
  };
}

async function submit() {
  if (!/^[A-Z]$/i.test(command.value.letter)) {
    message.error("Letter must be A-Z");
    return;
  }

  for (const g of command.value.items) {
    for (const f of g.fields) {
      if (f.type === "bool") delete (f as any).base;
      if (f.type !== "bool" && (f.base !== 10 && f.base !== 16)) {
        message.error(`Field "${f.name}" has invalid base`);
        return;
      }
    }
  }

  try {
    await invoke("append_command", {
      newCmd: {
        letter: command.value.letter.toUpperCase(),
        description: command.value.description,
        items: command.value.items
      }
    });
    message.success("Command saved!");
    reset();
  } catch (err: any) {
    message.error(String(err));
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
