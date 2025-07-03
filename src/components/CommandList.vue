<template>
  <!-- File uploader -->
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

  <!-- Wrapper fills all remaining space in the sider -->
  <div class="list-wrapper scroll-hide">
    <n-list class="frame-list">
      <n-list-item
          v-for="(f,i) in frames"
          :key="i"
          :class="{ selected: i === sel }"
          @click="select(i)"
      >
        {{ f }}
      </n-list-item>
    </n-list>
  </div>
</template>


<script setup lang="ts">
import { ref } from "vue";
import { NUpload, NUploadDragger, NList, NListItem } from "naive-ui";
import type { UploadFileInfo } from "naive-ui";
import { useSharedDecode } from "../composables/useSharedDecode";

const dummy  = ref<UploadFileInfo[]>([]);
const frames = ref<string[]>([]);
const sel    = ref(-1);
const { run } = useSharedDecode();

function clean(line: string) {
  return line.split(/\s+/)[0];           // cut after first space
}
async function onFile(info: { file: UploadFileInfo }) {
  const txt = await info.file.file?.text();
  frames.value = txt
      ?.split(/[\r\n]+/)
      .map(l => clean(l.trim()))
      .filter(f => f.startsWith("<") && f.length > 0) || [];
  sel.value = -1;
}
function select(i: number) { sel.value = i; run(frames.value[i]); }
</script>

<style scoped>
.list-wrapper {
  flex: 1 1 auto;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  scrollbar-width: none;
}
.list-wrapper::-webkit-scrollbar {
  display: none;
}
.frame-list {
  flex-grow: 1;
}
.selected {
  background-color: var(--n-item-color-active);
}

</style>
