<script setup lang="ts">
import { computed, ref } from "vue";
import { RouterView } from 'vue-router'
import {
  darkTheme,
  enUS,
  dateEnUS,
  frFR,
  dateFrFR,
  NConfigProvider,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NMessageProvider
} from 'naive-ui'
import Header from './components/base/Header.vue'
import { useLocaleStore } from "./shared";

const theme = ref<typeof darkTheme | null>(darkTheme);
const localeStore = useLocaleStore();

const toggleTheme = () => {
  theme.value = theme.value === null ? darkTheme : null;
};

const naiveLocale = computed(() => (localeStore.locale === "fr" ? frFR : enUS));
const naiveDateLocale = computed(() => (localeStore.locale === "fr" ? dateFrFR : dateEnUS));
</script>

<template>
  <NConfigProvider :theme="theme" :locale="naiveLocale" :date-locale="naiveDateLocale">
    <NMessageProvider placement="bottom-left">
      <NLayout position="absolute" class="app-shell">
        <NLayoutHeader bordered class="app-header">
          <Header @toggle-theme="toggleTheme" :theme="theme" />
        </NLayoutHeader>
        <NLayoutContent content-style="padding: 20px 24px 28px; max-width: 1200px; margin: 0 auto; min-height: calc(100dvh - 64px);">
          <RouterView v-slot="{ Component }">
            <Transition name="page-fade" mode="out-in">
              <component :is="Component" />
            </Transition>
          </RouterView>
        </NLayoutContent>
      </NLayout>
    </NMessageProvider>
  </NConfigProvider>
</template>

<style scoped>
.app-shell {
  min-height: 100dvh;
  background: linear-gradient(180deg, rgba(96, 165, 250, 0.08), rgba(15, 23, 42, 0) 180px);
}

.app-header {
  backdrop-filter: blur(6px);
}

.page-fade-enter-active,
.page-fade-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(10px) scale(0.98);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.98);
}
</style>