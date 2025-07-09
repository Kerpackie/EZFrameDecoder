<template>
  <n-config-provider :theme="isDarkMode ? darkTheme : null" :theme-overrides="isDarkMode ? darkThemeOverrides : lightThemeOverrides">
    <n-dialog-provider>
      <n-message-provider>
        <n-layout class="root">

          <!-- Header -->
          <n-layout-header class="header" bordered>
            <h2 class="logo" @click="router.push('/')">EZ Frame Decoder</h2>
          </n-layout-header>

          <!-- Body -->
          <n-layout has-sider class="body-row">

            <!-- Sidebar -->
            <n-layout-sider
                :width="220"
                :collapsed-width="60"
                :collapsed="collapsed"
                collapse-mode="width"
                bordered
                class="nav-sider"
                @collapse="collapsed = true"
                @expand="collapsed = isMedium"
            >
              <n-menu
                  :collapsed="collapsed"
                  :options="menuOptions"
                  :value="active"
                  @update:value="nav"
              />
            </n-layout-sider>

            <!-- Main Content (now a separate component) -->
            <n-layout-content class="main">
              <MainContent />
            </n-layout-content>

          </n-layout>
        </n-layout>
      </n-message-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import {
  NConfigProvider,
  NDialogProvider,
  NMessageProvider,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NLayoutSider,
  NMenu,
  NIcon
} from "naive-ui";
import {h, ref, watch, computed, watchEffect} from "vue";
import {useRoute, useRouter} from "vue-router";
import { lightThemeOverrides, darkThemeOverrides, darkTheme } from "./theme";
import {useBreakpoint} from "./composables/useBreakpoint";
import { useSettingsStore } from './stores/settingsStore';
import MainContent from './MainContent.vue'; // Import the new MainContent component

import {
  CodeSlashOutline,
  BuildOutline,
  InformationCircleOutline,
  EyeOutline,
  SettingsOutline
} from "@vicons/ionicons5";

const icon = (comp: any) => () => h(NIcon, null, {default: () => h(comp)});

const router = useRouter();
const route = useRoute();
const { isAdvancedMode, isDarkMode } = useSettingsStore();

// --- THEME-AWARE SCROLLBAR ---
watchEffect(() => {
  const thumbColor = isDarkMode.value ? '#4b5563' : '#cbd5e1';
  const trackColor = isDarkMode.value ? '#1f2937' : '#f1f5f9';
  document.documentElement.style.setProperty('--scrollbar-thumb-color', thumbColor);
  document.documentElement.style.setProperty('--scrollbar-track-color', trackColor);
});

const menuOptions = computed(() => {
  const standardMenu = [
    {label: "Decoder", key: "/", icon: icon(CodeSlashOutline)},
    {label: "View Commands", key: "/view-commands", icon: icon(EyeOutline)},
  ];

  const advancedMenu = [
    {label: "Spec Editor", key: "/spec-editor", icon: icon(BuildOutline)},
  ];

  const commonMenu = [
    {label: "Settings", key: "/settings", icon: icon(SettingsOutline)}
  ];

  if (isAdvancedMode.value) {
    return [...standardMenu.slice(0, 1), ...advancedMenu, ...commonMenu];
  }
  return [...standardMenu, ...commonMenu];
});

const active = ref(route.path);
watch(() => route.path, p => (active.value = p));

watch(() => route.fullPath, () => {
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      window.scrollTo({ top: 0, behavior: 'auto' })
    })
  })
})

function nav(key: string) {
  router.push(key);
}

const isMedium = useBreakpoint("(max-width: 1200px)");
const collapsed = ref(isMedium.value);
watch(isMedium, val => (collapsed.value = val));
</script>

<style>
/* GLOBAL SCROLLBAR STYLES */
/* For Firefox */
* {
  scrollbar-width: thin;
  scrollbar-color: var(--scrollbar-thumb-color) var(--scrollbar-track-color);
}

/* For Chrome, Edge, and Safari */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}
::-webkit-scrollbar-track {
  background: var(--scrollbar-track-color);
}
::-webkit-scrollbar-thumb {
  background-color: var(--scrollbar-thumb-color);
  border-radius: 4px;
  border: 2px solid var(--scrollbar-track-color);
}
</style>

<style scoped>
/* Layout */
.root {
  height: 100vh;
}

.header {
  height: 48px;
  display: flex;
  align-items: center;
  padding: 0 16px;
}

.logo {
  cursor: pointer;
  margin: 0;
  font-weight: 700;
}

.body-row {
  height: calc(100vh - 48px);
}

.main {
  height: 100%;
  overflow: auto;
}

.main :deep(.n-layout) {
  height: 100%;
}

.nav-sider :deep(.n-icon) {
  padding-left: 13px;
}
</style>
