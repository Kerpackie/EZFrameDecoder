<template>
  <n-config-provider :theme="darkTheme">
    <n-layout>
      <n-layout-header bordered>
        <h2 class="p-2">Frame Decoder</h2>
      </n-layout-header>

      <n-layout has-sider>
        <n-layout-sider width="300">
          <command-list />
        </n-layout-sider>

        <n-layout-content class="p-4">
          <decoder-input />
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
  NLayoutContent
} from "naive-ui";
import DecoderInput from "./components/DecoderInput.vue";
import CommandList from "./components/CommandList.vue";
</script>

<style>
body {
  margin: 0;
}
</style>
