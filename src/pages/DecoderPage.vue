<template>
  <n-layout has-sider class="decoder-layout">
    <!-- Command list sidebar -->
    <n-layout-sider
        :width="isMedium ? 220 : 310"
        :collapsed-width="isMedium ? 0 : 220"
        :collapsed="collapsed"
        :show-trigger="isMedium"
        collapse-mode="width"
        bordered
        class="cmd-sider"
        @collapse="collapsed = true"
        @expand="collapsed = false"
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
import { ref, watch } from "vue";
import { NLayout, NLayoutSider, NLayoutContent } from "naive-ui";
import FrameList   from "../components/FrameList.vue";
import DecoderInput  from "../components/DecoderInput.vue";
import DecodedPane   from "../components/DecodedPane.vue";
import { useBreakpoint } from "../composables/useBreakpoint";

const isMedium = useBreakpoint("(max-width: 1200px)");
const collapsed = ref(isMedium.value);

watch(isMedium, val => {
  collapsed.value = val; // auto collapse when resizing small
});
</script>

<style scoped>
.decoder-layout { height: 100%; }

.cmd-sider {
  transition: width 0.3s ease;
  background-color: #f8f9fa;
}

.decode-main {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 0 1rem;
}

.result {
  flex: 1 1 auto;
  overflow: auto;
}
</style>
