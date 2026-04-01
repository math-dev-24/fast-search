<script setup lang="ts">
import { computed } from "vue";
import { NSpace, NButton, NIcon, NPopover, NSelect, NTooltip } from "naive-ui";
import { RouterLink, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import Setting from "../Setting.vue";
import Details from "../sync/Details.vue";
import Indicator from "../sync/Indicator.vue";
import { useSync } from "../../composables/useSync.ts";
import ColorMode from "../ColorMode.vue";
import { darkTheme } from "naive-ui";
import { HomeOutline, BarChartOutline, LanguageOutline } from "@vicons/ionicons5";
import { useLocaleStore } from "../../shared";
import type { AppLocale } from "../../i18n";

defineProps<{
  theme: typeof darkTheme | null;
}>();

const { t } = useI18n();
const localeStore = useLocaleStore();
const router = useRouter();

const localeOptions = computed(() => [
  { label: t("language.fr"), value: "fr" },
  { label: t("language.en"), value: "en" },
]);

const routeIconMap: Record<string, unknown> = {
  "/": HomeOutline,
  "/statistique": BarChartOutline,
};

const navigationRoutes = computed(() =>
  router.options.routes
    .filter((route) => route.path && route.meta?.labelKey)
    .map((route) => ({
      path: route.path,
      name: t(String(route.meta?.labelKey)),
      icon: routeIconMap[route.path] ?? HomeOutline,
    })),
);

const handleLocaleChange = (value: AppLocale) => {
  localeStore.setLocale(value);
};

const {
  inSync,
  hasError,
  hasSuccess,
  overallProgress,
  progressStatus,
  statusIcon,
  syncSummary,
  processDetails,
  startSync,
} = useSync();

const emit = defineEmits<{
  (e: "toggle-theme"): void;
}>();
</script>

<template>
  <header class="header-shell">
    <NSpace justify="space-between" align="center" class="px-6 py-3">
      <NSpace align="center" :size="12">
        <h1 class="text-lg font-semibold">{{ t("common.appName") }}</h1>
        <RouterLink
          v-for="route in navigationRoutes"
          :key="route.path"
          :to="route.path"
          custom
          v-slot="{ navigate, isActive }"
        >
          <NButton quaternary :type="isActive ? 'primary' : 'default'" @click="navigate">
            <template #icon>
              <NIcon size="16">
                <component :is="route.icon" />
              </NIcon>
            </template>
            {{ route.name }}
          </NButton>
        </RouterLink>
      </NSpace>

      <NSpace align="center" :size="8">
        <NTooltip trigger="hover">
          <template #trigger>
            <NSelect
              class="w-34"
              size="small"
              :value="localeStore.locale"
              :options="localeOptions"
              @update:value="handleLocaleChange"
            >
              <template #arrow>
                <NIcon><LanguageOutline /></NIcon>
              </template>
            </NSelect>
          </template>
          {{ t("language.label") }}
        </NTooltip>

        <ColorMode @toggle-theme="emit('toggle-theme')" :theme="theme" />

        <NPopover trigger="hover" :disabled="!inSync && !hasError && !hasSuccess" placement="bottom-end">
          <template #trigger>
            <NButton @click="startSync" :loading="inSync" tertiary round :type="progressStatus" :disabled="inSync">
              <template #icon>
                <NIcon size="16">
                  <component :is="statusIcon" />
                </NIcon>
              </template>
              {{ t("header.sync") }}
            </NButton>
          </template>

          <Details
            :inSync="inSync"
            :hasError="hasError.length > 0"
            :hasSuccess="hasSuccess"
            :overallProgress="overallProgress"
            :progressStatus="progressStatus"
            :syncSummary="syncSummary"
            :processDetails="processDetails"
            :statusIcon="statusIcon"
          />
        </NPopover>

        <Setting :inSync="inSync" />
      </NSpace>
    </NSpace>

    <Indicator
      :inSync="inSync"
      :hasError="hasError.length > 0"
      :hasSuccess="hasSuccess"
      :overallProgress="overallProgress"
      :progressStatus="progressStatus"
      :syncSummary="syncSummary"
    />
  </header>
</template>

<style scoped>
.header-shell {
  border-bottom: 1px solid rgba(148, 163, 184, 0.25);
}
</style>