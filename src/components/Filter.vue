<template>
    <NCard class="p-4">
        <NFlex vertical align="start">
            <NAutoComplete v-model:value="modelValue.searchResult" placeholder="Filtrer les résultats..."
                :options="options" clearable class="w-full">
                <template #prefix>
                    <NIcon size="16">
                        <Filter />
                    </NIcon>
                </template>
            </NAutoComplete>
            <NFlex justify="space-around">
                <NSwitch v-model:value="modelValue.showPath">
                    <template #checked>
                        <div class="flex items-center space-x-2">
                            <NIcon size="16">
                                <Eye />
                            </NIcon>
                            <span class="text-sm font-medium">Chemin visible</span>
                        </div>
                    </template>
                    <template #unchecked>
                        <div class="flex items-center space-x-2">
                            <NIcon size="16">
                                <EyeOff />
                            </NIcon>
                            <span class="text-sm font-medium">Chemin masqué</span>
                        </div>
                    </template>
                </NSwitch>

                <NCheckbox v-model:checked="modelValue.searchInPath" label="Rechercher dans le chemin" />
            </NFlex>
        </NFlex>
    </NCard>
</template>

<script setup lang="ts">
import { NSwitch, NAutoComplete, NCheckbox, NIcon, NCard, NFlex } from 'naive-ui';
import { Filter, Eye, EyeOff } from '@vicons/ionicons5';

const modelValue = defineModel<{
    searchResult: string;
    searchInPath: boolean;
    showPath: boolean;
}>({
    required: true
});

defineProps<{
    options: { label: string; value: string }[];
}>();

</script>