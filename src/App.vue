<template>
  <n-config-provider :theme-overrides="themeOverrides">
    <n-message-provider>
      <n-layout class="root">
        <n-layout-header class="header" bordered>
          <h2 @click="router.push('/')" style="cursor: pointer">EZ Frame Decoder</h2>
        </n-layout-header>

        <n-layout has-sider class="body-row">
          <n-layout-sider width="250" bordered class="side">
            <n-menu
                :options="menuOptions"
                :value="activeMenu"
                @update:value="navigate"
            />
          </n-layout-sider>

          <n-layout-content class="main">
            <router-view />
          </n-layout-content>
        </n-layout>
      </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { NConfigProvider, NLayout, NLayoutHeader, NLayoutContent, NLayoutSider, NMessageProvider, NMenu } from 'naive-ui';
import { themeOverrides } from './theme';

const router = useRouter();
const route = useRoute();

const menuOptions = [
  { label: 'Decode', key: '/' },
  { label: 'Add Command', key: '/add-command' },
  { label: 'About', key: '/about' }
];

const activeMenu = ref(route.path);
watch(() => route.path, (newPath) => {
  activeMenu.value = newPath;
});

function navigate(key: string) {
  router.push(key);
}
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
.side {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.main {
  padding: 1rem;
  height: 100%;
  overflow: auto;
}
</style>
