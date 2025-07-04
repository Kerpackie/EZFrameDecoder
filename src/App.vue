<template>
  <n-config-provider :theme-overrides="abrelTheme">
    <n-message-provider>
      <!-- root fills viewport -->
      <n-layout class="root">

        <!-- fixed header (48 px) -->
        <n-layout-header class="header" bordered>
          <h2 class="flex-1">EZ Frame Decoder</h2>
            <div style="margin-left:auto;">
            <n-button type="info" quaternary size="small" @click="toggleAbout">
              {{ showAbout ? "Decoder" : "About" }}
            </n-button>
            </div>
        </n-layout-header>

        <!-- flex row below header (fills the rest) -->
        <n-layout class="body-row" has-sider>

          <n-layout-sider width="310" bordered class="side">
            <command-list/>
          </n-layout-sider>

          <n-layout-content class="main">
            <decoder-input v-if="!showAbout" />
            <decoded-pane  v-if="!showAbout" class="result scroll-hide" />

            <about-pane    v-else class="result scroll-hide" />
          </n-layout-content>

        </n-layout>
      </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import {
  NLayout,
  NLayoutHeader,
  NLayoutSider,
  NLayoutContent,
  NConfigProvider,
} from "naive-ui";
import { ref } from "vue";
import CommandList from "./components/CommandList.vue";
import DecoderInput from "./components/DecoderInput.vue";
import DecodedPane from "./components/DecodedPane.vue";
import {abrelTheme} from "./theme/abrelTheme";

import AboutPane from "./components/AboutPane.vue";

const showAbout = ref(false);
function toggleAbout() { showAbout.value = !showAbout.value; }
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
  height: calc(100% - 1rem);
  display: flex;
  flex-direction: column;
  margin-left: 1rem;
  margin-right: 1rem;
  margin-top: 1rem;
}

.result {
  flex: 1 1 auto;
  overflow: auto;
}
</style>

