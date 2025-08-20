<script setup lang="ts">
import { NBadge, NCollapse, NCollapseItem, NGi, NGrid, NTabs, NTabPane, NButton, NTable, NCard } from "naive-ui";
import type { File } from "../../types";
import Card from "./CardFile.vue";
import Line from "./LineFile.vue";
import { ref } from "vue";

defineProps<{
  files: File[]
}>()

const emit = defineEmits<{
  (e: 'openFile', path: string): void,
  (e: 'copyPath', path: string): void,
  (e: 'previewFile', file: File): void,
}>();

const maxFilesCard = ref<number>(8);
const maxFilesLine = ref<number>(15);

</script>

<template>
  <NCard v-if="files.length > 0">
    <NCollapse class="mb-6">
      <NCollapseItem name="Fichiers">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div>📄 Fichiers</div>
            <NBadge
                :max="999"
                :value="files.length"
                type="info"
            />
          </div>
        </template>
        <NTabs animated type="segment">
          <NTabPane name="card" tab="Board">
            <NGrid :cols="4" :x-gap="12" :y-gap="12" responsive="screen">
              <NGi
                  v-for="file in files.slice(0, maxFilesCard)"
                  :key="file.name"
              >
                <Card
                    :file="file"
                    @openFile="emit('openFile', file.path)"
                />
              </NGi>
            </NGrid>
            <div
                v-if="files.length > maxFilesCard"
                class="mt-4 text-center"
            >
              <NButton
                  class="w-full max-w-md"
                  secondary
                  size="large"
                  type="primary"
                  @click="maxFilesCard += 10"
              >
                📄 Voir {{ Math.min(10, files.length - maxFilesCard) }}
                fichiers de plus
              </NButton>
            </div>
          </NTabPane>
          <NTabPane name="line" tab="Tableau">
            <NTable :single-line="false" class="w-full">
              <thead>
              <tr>
                <th>Nom</th>
                <th>Path</th>
                <th>Create at</th>
                <th>Accessed at</th>
                <th>Action</th>
              </tr>
              </thead>
              <tbody>
              <Line
                  v-for="file in files.slice(0, maxFilesLine)"
                  :key="file.name"
                  :file="file"
                  @openFile="emit('openFile', file.path)"
                  @previewFile="emit('previewFile', file)"
                  @copyPath="emit('copyPath', file.path)"
              />
              <tr v-if="files.length > maxFilesLine" class="border-t border-gray-200">
                <td colspan="3" class="text-center">
                  <NButton @click="maxFilesLine += 10" tertiary>
                    📄 Voir {{ Math.min(10, files.length - maxFilesLine) }}
                    fichiers de plus
                  </NButton>
                </td>
              </tr>
              </tbody>
            </NTable>
          </NTabPane>
        </NTabs>
      </NCollapseItem>
    </NCollapse>
  </NCard>
</template>