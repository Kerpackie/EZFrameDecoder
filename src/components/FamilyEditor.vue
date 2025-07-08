<template>
  <n-modal v-model:show="showModal" preset="card" :title="isEdit ? 'Edit Family' : 'Create New Family'" style="width: 600px;">
    <n-form ref="formRef" :model="formModel" :rules="rules" label-placement="left" label-width="120">
      <n-form-item label="Name" path="name">
        <n-input v-model:value="formModel.name" placeholder="e.g., DriverBoard Commands" />
      </n-form-item>
      <n-form-item label="Description" path="description">
        <n-input v-model:value="formModel.description" placeholder="A short description of the family" type="textarea" />
      </n-form-item>
      <n-form-item label="Start Character(s)" path="start">
        <n-input v-model:value="formModel.start" placeholder="e.g., < or $" :disabled="isEdit" />
      </n-form-item>
      <n-form-item label="Terminator" path="terminator">
        <n-input v-model:value="formModel.terminator" placeholder="e.g., >" />
      </n-form-item>
      <n-form-item label="Frame Length" path="frame_len">
        <n-input-number v-model:value="formModel.frame_len" :min="1" placeholder="e.g., 23" style="width: 100%" />
      </n-form-item>
      <n-space justify="end">
        <n-button @click="showModal = false">Cancel</n-button>
        <n-button type="primary" @click="handleSubmit">
          {{ isEdit ? 'Save Changes' : 'Create' }}
        </n-button>
      </n-space>
    </n-form>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { NModal, NForm, NFormItem, NInput, NInputNumber, NButton, NSpace, useMessage, type FormInst, type FormRules } from 'naive-ui';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  show: boolean;
  familyData: object | null; // null for create, object for edit
}>();

const emit = defineEmits(['update:show', 'saved']);

const msg = useMessage();
const formRef = ref<FormInst | null>(null);
const showModal = ref(props.show);

const isEdit = computed(() => props.familyData !== null);

const createBlankFamily = () => ({
  name: '',
  description: '',
  start: '',
  terminator: '>',
  frame_len: 23,
  commands: [],
});

const formModel = ref(createBlankFamily());

watch(() => props.show, (newValue) => {
  showModal.value = newValue;
  if (newValue) {
    if (isEdit.value) {
      formModel.value = JSON.parse(JSON.stringify(props.familyData));
    } else {
      formModel.value = createBlankFamily();
    }
  }
});

watch(showModal, (newValue) => {
  if (!newValue) {
    emit('update:show', false);
  }
});

const rules: FormRules = {
  name: [{ required: true, message: 'Family name is required.', trigger: 'blur' }],
  start: [{ required: true, message: 'Start character is required.', trigger: 'blur' }],
  terminator: [{ required: true, message: 'Terminator is required.', trigger: 'blur' }],
  frame_len: [{ type: 'number', required: true, message: 'Frame length is required.', trigger: 'blur' }],
};

async function handleSubmit(e: MouseEvent) {
  e.preventDefault();
  formRef.value?.validate(async (errors) => {
    if (!errors) {
      try {
        if (isEdit.value) {
          await invoke('update_family', {
            originalStart: (props.familyData as any).start,
            fam: formModel.value,
          });
          msg.success('Family updated successfully!');
        } else {
          await invoke('create_family', { fam: formModel.value });
          msg.success('Family created successfully!');
        }
        emit('saved');
        showModal.value = false;
      } catch (err) {
        msg.error(String(err));
        console.error(err);
      }
    } else {
      msg.error('Please fill out all required fields.');
    }
  });
}
</script>
