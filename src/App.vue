<template>
  <n-config-provider :theme-overrides="themeOverrides">
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

            <!-- Main Content -->
            <n-layout-content class="main">
              <router-view/>
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
import {h, ref, watch} from "vue";
import {useRoute, useRouter} from "vue-router";
import {themeOverrides} from "./theme";
import {useBreakpoint} from "./composables/useBreakpoint";

import {
  CodeSlashOutline,
  BuildOutline,
  InformationCircleOutline
} from "@vicons/ionicons5";

const icon = (comp: any) => () => h(NIcon, null, {default: () => h(comp)});

const router = useRouter();
const route = useRoute();

const menuOptions = [
  {label: "Decoder", key: "/", icon: icon(CodeSlashOutline)},
  {label: "Spec Editor", key: "/spec-editor", icon: icon(BuildOutline)},
  {label: "About", key: "/about", icon: icon(InformationCircleOutline)}
];

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
  padding: 1rem;
  background-color: #f8f9fa; /* Added for visual consistency */
}

/*
  FIX: This rule prevents double scrollbars.
  It forces the root layout of any page loaded via router-view
  to be constrained to the height of the .main container,
  overriding problematic styles like `height: 100vh` on the page itself.
*/
.main :deep(.n-layout) {
  height: 100%;
}

/* Padding for icons */
.nav-sider :deep(.n-icon) {
  padding-left: 13px;
}
</style>
