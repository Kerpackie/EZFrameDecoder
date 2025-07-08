<template>
  <n-layout has-sider class="decoder-layout">
    <!-- Command list sidebar -->
    <n-layout-sider
        :width="240"
        :collapsed-width="0"
        :collapsed="collapsed"
        show-trigger
        collapse-mode="width"
        bordered
        class="cmd-sider"
        @update:collapsed="v => collapsed = v"
    >
      <frame-list />
    </n-layout-sider>

    <!-- Main content -->
    <n-layout-content class="decode-main">
      <decoder-input />
      <decoded-pane class="result scroll-hide" />
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NLayout, NLayoutSider, NLayoutContent } from "naive-ui";
import FrameList   from "../components/FrameList.vue";
import DecoderInput  from "../components/DecoderInput.vue";
import DecodedPane   from "../components/DecodedPane.vue";

const collapsed = ref(false);
</script>

<style scoped>
.decoder-layout { height: 100%; }

.cmd-sider {
  transition: width 0.3s ease;
  border-radius: 0 8px 8px 0;
}
.cmd-sider :deep(.n-layout-sider-content) {
  padding: 1rem;
}


.decode-main {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
}

.result {
  flex: 1 1 auto;
  overflow: auto;
}
</style>
