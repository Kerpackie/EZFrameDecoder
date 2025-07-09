<template>
  <n-card size="small" title="Switch Block" closable @close="onRemove?.()">
    <!-- ────────────────── Switch key ────────────────── -->
    <n-form label-width="100" class="mb-3">
      <n-form-item label="Switch Key">
        <n-select
            v-model:value="item.switch"
            :options="selectOptions"
            placeholder="Select Header Field"
        />
      </n-form-item>
    </n-form>

    <!-- ────────────────── CASE TABS ────────────────── -->
    <n-tabs type="line" animated>
      <!-- Each case tab -->
      <n-tab-pane
          v-for="(caseDef, key) in item.cases"
          :key="key"
          :name="key"
      >
        <!-- === tab label with close icon === -->
        <template #tab>
          {{ key }}
          <n-icon
              size="14"
              class="ml-1 text-red-500"
              @click.stop="removeCase(key)"
          >
            <close-outline />
          </n-icon>
        </template>

        <!-- ---------- Case meta ---------- -->
        <n-form label-width="100" class="mb-2">
          <n-form-item label="Description">
            <n-input v-model:value="caseDef.description" />
          </n-form-item>
        </n-form>

        <!-- ---------- Groups inside this case ---------- -->
        <div v-for="(grp, gidx) in caseDef.groups" :key="gidx" class="mb-3">
          <n-card
              size="small"
              :title="grp.name || `Group ${gidx + 1}`"
              closable
              @close="removeGroup(caseDef, gidx)"
          >
            <n-form label-width="100" class="mb-2">
              <n-form-item label="Group Name">
                <n-input v-model:value="grp.name" />
              </n-form-item>
            </n-form>
            <!-- Field editor for this group -->
            <FieldEditor v-model:fields="grp.fields" />
          </n-card>
        </div>

        <n-button
            size="tiny"
            quaternary
            @click="addGroup(caseDef)"
            class="mb-4"
        >
          + Add Group
        </n-button>
      </n-tab-pane>

      <!-- Default tab -->
      <n-tab-pane v-if="item.default" name="default">
        <template #tab>
          Default
          <n-icon
              size="14"
              class="ml-1 text-red-500"
              @click.stop="item.default = null"
          >
            <close-outline />
          </n-icon>
        </template>

        <!-- ---------- Default meta ---------- -->
        <n-form label-width="100" class="mb-2">
          <n-form-item label="Description">
            <n-input v-model:value="item.default.description" />
          </n-form-item>
        </n-form>

        <!-- ---------- Groups in default ---------- -->
        <div
            v-for="(grp, gidx) in item.default.groups"
            :key="gidx"
            class="mb-3"
        >
          <n-card
              size="small"
              :title="grp.name || `Group ${gidx + 1}`"
              closable
              @close="removeGroup(item.default, gidx)"
          >
            <n-form label-width="100" class="mb-2">
              <n-form-item label="Group Name">
                <n-input v-model:value="grp.name" />
              </n-form-item>
            </n-form>
            <FieldEditor v-model:fields="grp.fields" />
          </n-card>
        </div>

        <n-button
            size="tiny"
            quaternary
            @click="addGroup(item.default)"
            class="mb-4"
        >
          + Add Group
        </n-button>
      </n-tab-pane>
    </n-tabs>

    <!-- ────────────────── Add case / default row ────────────────── -->
    <div class="mt-3 flex gap-2">
      <n-input
          v-model:value="newCaseKey"
          placeholder="New case key (e.g. 01 or ABCD)"
          size="small"
          @keyup.enter="addCase"
      />
      <n-button size="small" @click="addCase">Add Case</n-button>
      <n-button size="small" @click="addDefault" :disabled="!!item.default">
        Add Default
      </n-button>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { CloseOutline } from '@vicons/ionicons5'
import FieldEditor from './FieldEditor.vue'
import { NCard, NForm, NFormItem, NInput, NTabs, NTabPane, NIcon, NButton, useMessage, NSelect } from 'naive-ui'

/* ---------- props ---------- */
const props = defineProps<{
  item: {
    kind: 'switch'
    switch: string
    cases: Record<string, SwitchCase>
    default: SwitchCase | null
  }
  headerFields: string[]
  onRemove?: () => void
}>()

interface SwitchCase {
  description?: string
  groups: Group[]
}
interface Group {
  name: string
  fields: any[]
}

/* ---------- local state ---------- */
const newCaseKey = ref('')
const msg = useMessage()

/* ---------- Computed options for dropdown ---------- */
const selectOptions = computed(() =>
    props.headerFields.map(field => ({
      label: field,
      value: field
    }))
)

/* ---------- case helpers ---------- */
function addCase () {
  let key = newCaseKey.value.trim()
  if (!key) return;

  // Auto-prefix with '0x' if it's not already there
  if (!key.toLowerCase().startsWith('0x')) {
    key = '0x' + key;
  }

  if (props.item.cases[key]) {
    msg.warning(`Case key "${key}" already exists.`);
    return;
  }

  props.item.cases[key] = { description: '', groups: [] }
  newCaseKey.value = ''
}
function removeCase (key: string) {
  delete props.item.cases[key]
}

/* ---------- group helpers ---------- */
function createEmptyGroup (): Group {
  return { name: 'payload', fields: [] }
}
function addGroup (target: SwitchCase) {
  target.groups.push(createEmptyGroup())
}
function removeGroup (target: SwitchCase, idx: number) {
  target.groups.splice(idx, 1)
}

/* ---------- default helpers ---------- */
function addDefault () {
  if (!props.item.default) {
    props.item.default = { description: '', groups: [] }
  }
}
</script>
