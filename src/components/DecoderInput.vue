<template>
  <n-input
      v-model:value="frame"
      placeholder="Paste a 23-byte frame here"
      class="mb-2"
  />
  <n-button type="primary" @click="run(frame)">Decode</n-button>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NInput, NButton } from "naive-ui";
import { useSharedDecode } from "../composables/useSharedDecode";

const frame = ref("");
const { run } = useSharedDecode();
</script>
