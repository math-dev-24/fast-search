<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NSpace, NButton } from 'naive-ui';
import { RouterLink } from 'vue-router';
import Setting from './Setting.vue';

const router = useRouter();
const routes = router.getRoutes();
</script>

<template>
    <header>
        <NSpace justify="space-between" align="center" class="px-8 py-4">
            <h1 class="text-2xl font-bold">
                Fast Search
            </h1>
            <NSpace align="center">
                <RouterLink v-for="route in routes" :key="route.path" :to="route.path" custom v-slot="{ navigate, isActive }">
                    <NButton :class="{ 'active': isActive }" @click="navigate">
                        {{ route.name }}
                    </NButton>
                </RouterLink>
                <Setting />
            </NSpace>
        </NSpace>
    </header>
</template>