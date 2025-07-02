<template>
  <n-button secondary @click="reload">Reload Spec</n-button>
</template>

<script setup lang="ts">
import { NButton, useMessage } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
const message = useMessage();

async function reload() {
  try {
    await invoke("reload_spec");
    message.success("Spec reloaded!");
  } catch (e: any) {
    message.error(String(e));
  }
}
</script>