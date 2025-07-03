<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
    <!-- root fills viewport -->
    <n-layout class="root">

      <!-- fixed header (48 px) -->
      <n-layout-header class="header" bordered>
        <h2>Frame Decoder</h2>
      </n-layout-header>

      <!-- flex row below header (fills the rest) -->
      <n-layout class="body-row" has-sider>

        <n-layout-sider width="310" bordered class="side">
          <command-list/>
        </n-layout-sider>

        <n-layout-content class="main">
          <decoder-input/>
          <decoded-pane class="result scroll-hide"/>
        </n-layout-content>

      </n-layout>
    </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import {
  darkTheme,
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NConfigProvider,
} from "naive-ui";
import CommandList from "./components/CommandList.vue";
import DecoderInput from "./components/DecoderInput.vue";
import DecodedPane from "./components/DecodedPane.vue";
</script>

<style scoped>
.root {
  height: 100vh;
}

.header {
  height: 48px;
  display: flex;
  align-items: center;
  padding: 0 16px;
}

.body-row {
  height: calc(100vh - 48px);
}

/* subtract header */
.side {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.main {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.result {
  flex: 1 1 auto;
  overflow: auto;
}
</style>
