<script setup lang="ts">
import { Sunny, Moon } from '@vicons/ionicons5'
import { NButton, NIcon } from 'naive-ui'
import { darkTheme } from 'naive-ui'

defineProps<{
    theme: typeof darkTheme | null
}>()

const toggleTheme = () => {
    emit('toggle-theme')
}

const emit = defineEmits(['toggle-theme'])

</script>

<template>
    <NButton @click="toggleTheme" tertiary round :type="theme === null ? 'primary' : 'default'">
        <template #icon>
            <NIcon size="16">
                <Sunny v-if="theme === null" />
                <Moon v-else />
            </NIcon>
        </template>
    </NButton>
</template>