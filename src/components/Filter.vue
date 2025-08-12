<template>
    <NCard class="p-4">
        <NFlex vertical align="start">
            <NAutoComplete 
                v-model:value="searchStore.search" 
                placeholder="Filtrer les résultats..."
                :options="searchStore.options" 
                clearable 
                class="w-full"
            >
                <template #prefix>
                    <NIcon size="16">
                        <Filter />
                    </NIcon>
                </template>
            </NAutoComplete>
            <NFlex justify="space-around" align="center">
                <NButton @click="searchStore.exportToCsv" tertiary type="info">
                    Exporter en CSV
                </NButton>
                <NCheckbox v-model:checked="searchStore.query.search_in_content" label="Rechercher dans le chemin" />
            </NFlex>
        </NFlex>
    </NCard>
</template>

<script setup lang="ts">
import { NAutoComplete, NCheckbox, NIcon, NCard, NFlex, NButton } from 'naive-ui';
import { Filter } from '@vicons/ionicons5';
import { useSearchStore } from '../shared';

const searchStore = useSearchStore();

</script>