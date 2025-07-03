<template>
  <div class="frame-row">
    <n-input
        v-model:value="frame"
        placeholder="Paste a 23-byte frame here"
    />
    <n-button type="primary" @click="run(frame)">Decode</n-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NInput, NButton } from "naive-ui";
import { useSharedDecode } from "../composables/useSharedDecode";

const frame = ref("");
const { run } = useSharedDecode();
</script>

<style scoped>
.frame-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.frame-row n-input {
  flex: 1 1 auto;
}
</style>
