<template>
  <n-card>
    <n-upload
        v-model:file-list="dummyFiles"
        :default-upload="false"
        accept=".txt"
        :on-change="onFile"
        :max="1"
    >
      <n-upload-dragger>
        <div style="padding: 32px; text-align: center;">
          Drop a text file with one frame per line
        </div>
      </n-upload-dragger>
    </n-upload>

    <n-list bordered>
      <n-list-item
          v-for="(f, i) in frames"
          :key="i"
          @click="select(i)"
          :class="{ selected: i === selIdx }"
      >
        {{ f }}
      </n-list-item>
    </n-list>

    <decoded-view
        v-if="selIdx !== -1"
        :result="decoded"
        :error="error"
    />
  </n-card>
</template>

<script setup lang="ts">
import {
  NCard,
  NUpload,
  NUploadDragger,
  NList,
  NListItem,
} from "naive-ui";
import { ref } from "vue";
import type { UploadFileInfo } from "naive-ui";
import { useDecode } from "../composables/useDecode";
import DecodedView from "./DecodedView.vue";

const dummyFiles = ref<UploadFileInfo[]>([]);   // 🔹 keeps Naive stateful
const frames = ref<string[]>([]);
const selIdx = ref(-1);

const { decoded, error, run } = useDecode();

async function onFile(info: { file: UploadFileInfo }) {
  if (info.file.file) {
    const text = await info.file.file.text();
    frames.value = text
        .split(/[\r\n]+/)
        .map((l) => l.trim())
        .filter(Boolean);
    selIdx.value = -1;
  }
}

function select(i: number) {
  selIdx.value = i;
  run(frames.value[i]);
}
</script>

<style scoped>
.selected {
  background-color: var(--n-color-primary-6);
}
</style>
