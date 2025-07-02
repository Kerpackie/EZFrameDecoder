<template>
  <n-upload
      v-model:file-list="dummy"
      :default-upload="false"
      accept=".txt"
      :max="1"
      :on-change="onFile"
  >
    <n-upload-dragger>
      Drop or click to load a text file
    </n-upload-dragger>
  </n-upload>

  <n-list class="scrollable">
    <n-list-item
        v-for="(f,i) in frames"
        :key="i"
        :class="{ selected: i===sel }"
        @click="select(i)"
    >
      {{ f }}
    </n-list-item>
  </n-list>
</template>

<script setup lang="ts">
import {
  NUpload, NUploadDragger,
  NList, NListItem
} from "naive-ui";
import { ref } from "vue";
import type { UploadFileInfo } from "naive-ui";
import { useSharedDecode } from "../composables/useSharedDecode";

const dummy = ref<UploadFileInfo[]>([]);
const frames = ref<string[]>([]);
const sel = ref(-1);
const { run } = useSharedDecode();

async function onFile(info: { file: UploadFileInfo }) {
  const text = await info.file.file?.text();
  frames.value = text
      ?.split(/[\r\n]+/)
      .map(l => l.trim())
      .filter(Boolean) || [];
  sel.value = -1;
}

function select(i: number) {
  sel.value = i;
  run(frames.value[i]);
}
</script>

<style scoped>
.scrollable {
  height: calc(100vh - 200px); /* adjust header size if needed */
  overflow-y: auto;
}
.selected { background: var(--n-item-color-active); }
</style>
