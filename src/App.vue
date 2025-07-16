<script setup lang="ts">
import { darkTheme, frFR, dateFrFR } from 'naive-ui'
import { NConfigProvider, NLayout, NLayoutHeader, NLayoutContent, NMessageProvider } from 'naive-ui'
import { RouterView } from 'vue-router'
import Header from './components/Header.vue'
import { ref } from 'vue'

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
          <RouterView />
        </NLayoutContent>
      </NLayout>
    </NMessageProvider>
  </NConfigProvider>
</template>