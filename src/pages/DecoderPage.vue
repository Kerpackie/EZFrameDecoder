<template>
  <n-layout has-sider class="decoder-page-layout">
    <n-layout-sider width="310" bordered class="side">
      <command-list />
    </n-layout-sider>

    <n-layout-content class="main-content">
      <decoder-input />
      <decoded-pane class="result scroll-hide" />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import { NLayout, NLayoutSider, NLayoutContent } from "naive-ui";
import CommandList from "../components/CommandList.vue";
import DecoderInput from "../components/DecoderInput.vue";
import DecodedPane from "../components/DecodedPane.vue";
</script>

<style scoped>
.decoder-page-layout {
  height: 100%;
  display: flex;
}

.side {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1 1 auto;
  padding: 1rem;
  display: flex;
  flex-direction: column;
}

.result {
  flex: 1 1 auto;
  overflow: auto;
}
</style>
