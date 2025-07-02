<template>
  <n-config-provider :theme="darkTheme">
    <n-layout class="root">
      <n-layout-header bordered class="px-4 py-2">
        <h2>Frame Decoder</h2>
      </n-layout-header>

      <n-layout has-sider>
        <!-- side list -->
        <n-layout-sider width="310" bordered>
          <command-list />
        </n-layout-sider>

        <!-- main column -->
        <n-layout-content class="p-4 main">
          <decoder-input />
          <decoded-pane class="result" />
        </n-layout-content>
      </n-layout>
    </n-layout>
  </n-config-provider>
</template>

<script setup lang="ts">
import {
  darkTheme,
  NConfigProvider,
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
} from "naive-ui";
import CommandList from "./components/CommandList.vue";
import DecoderInput from "./components/DecoderInput.vue";
import DecodedPane from "./components/DecodedPane.vue";
</script>

<style scoped>
.root, .main {
  height: 100vh;
}

.result {
  height: calc(100% - 60px); /* leave room for the input */
  overflow: auto;
}
</style>
