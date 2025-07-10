<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { NSpace, NButton, NIcon, NProgress } from 'naive-ui';
import { RouterLink } from 'vue-router';
import Setting from './Setting.vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { SyncCircle } from '@vicons/ionicons5';

const router = useRouter();
const routes = router.getRoutes();

const isSyncing = ref<boolean>(true);
const syncProgress = ref<number>(0);
const syncTotal = ref<number>(0);

const startSync = async () => {
    await invoke('sync_files_and_folders');
};

const valueProgress = computed(() => {
    return Math.round(syncProgress.value * 1000) / 10;
});

listen<void>('scan_files_started', () => {
    isSyncing.value = true;
});

listen<number>('scan_files_total', (event: any) => {
    syncTotal.value = event.payload;
});

listen<number>('scan_files_progress', (event: any) => {
    syncProgress.value = event.payload;
});

listen<string>('scan_files_finished', () => {
    isSyncing.value = false;
    syncProgress.value = 0;
});

</script>

<template>
    <header>
        <NSpace justify="space-between" align="center" class="px-8 py-4">
            <h1 class="text-2xl font-bold">
                Fast Search
            </h1>
            <NSpace align="center">
                <RouterLink v-for="route in routes" :key="route.path" :to="route.path" custom
                    v-slot="{ navigate, isActive }">
                    <NButton :class="{ 'active': isActive }" @click="navigate">
                        {{ route.name }}
                    </NButton>
                </RouterLink>
                <NButton @click="startSync" :loading="isSyncing" tertiary round type="info">
                    <template #icon>
                        <NIcon size="16">
                            <SyncCircle />
                        </NIcon>
                    </template>
                    Sync
                </NButton>
                <Setting />

            </NSpace>
        </NSpace>
        <NProgress v-show="isSyncing" type="line" :percentage="valueProgress" :show-indicator="false" :show-text="false" :height="2" />
    </header>
</template>