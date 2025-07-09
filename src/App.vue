<script setup lang="ts">
import { darkTheme, frFR, dateFrFR } from 'naive-ui'
import { NConfigProvider, NLayout, NLayoutHeader, NLayoutContent, NMessageProvider } from 'naive-ui'
import { RouterView } from 'vue-router'
import Header from './components/Header.vue'

</script>

<template>
  <NConfigProvider :theme="darkTheme" :locale="frFR" :date-locale="dateFrFR">
    <NMessageProvider placement="bottom-left">
      <NLayout position="absolute">
        <NLayoutHeader bordered>
          <Header></Header> 
        </NLayoutHeader>
        <NLayoutContent>
          <RouterView />
        </NLayoutContent>
      </NLayout>
    </NMessageProvider>
  </NConfigProvider>
</template>