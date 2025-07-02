<template>
  <n-card>
    <n-input v-model:value="frame" placeholder="Paste 23-byte frame here" />
    <n-button class="mt-2" type="primary" @click="decode">Decode</n-button>

    <decoded-view :result="decoded" :error="error" />
  </n-card>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NCard, NInput, NButton } from "naive-ui";
import { useDecode } from "../composables/useDecode";
import DecodedView from "./DecodedView.vue";

const frame = ref("");
const { decoded, error, run } = useDecode();
function decode() {
  run(frame.value);
}
</script>
