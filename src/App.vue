<script setup lang="ts">
import { ref, Transition } from 'vue'
import { RouterView } from 'vue-router'
import {
  darkTheme,
  frFR,
  dateFrFR,
  NConfigProvider,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NMessageProvider
} from 'naive-ui'
import Header from './components/base/Header.vue'

const theme = ref<typeof darkTheme | null>(darkTheme)

const toggleTheme = () => {
  theme.value = theme.value === null ? darkTheme : null
}
</script>

<template>
  <NConfigProvider :theme="theme" :locale="frFR" :date-locale="dateFrFR">
    <NMessageProvider placement="bottom-left">
      <NLayout position="absolute">
        <NLayoutHeader bordered>
          <Header @toggle-theme="toggleTheme" :theme="theme" />
        </NLayoutHeader>
        <NLayoutContent>
          <!-- Transition globale entre les pages -->
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