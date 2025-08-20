<script setup lang="ts">
import { NBadge, NCollapse, NCollapseItem, NGi, NGrid, NTabs, NTabPane, NButton, NTable, NCard } from "naive-ui";
import type { File } from "../../types";
import Card from "./CardFolder.vue";
import Line from "./LineFolder.vue";
import { ref } from "vue";

defineProps<{
  folders: File[]
}>()

const emit = defineEmits<{
  (e: 'openFile', path: string): void
}>()

const maxFoldersCard = ref<number>(8);
const maxFoldersLine = ref<number>(15);

</script>

<template>
  <NCard v-if="folders.length > 0">
    <NCollapse class="mb-6">
      <NCollapseItem name="folders">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div>📁 Dossiers</div>
            <NBadge
              :max="999"
              :value="folders.length"
              type="info"
            />
          </div>
        </template>
        <NTabs animated type="segment">
          <NTabPane name="card" tab="Board">
            <NGrid :cols="4" :x-gap="12" :y-gap="12" responsive="screen">
              <NGi
                v-for="folder in folders.slice(0, maxFoldersCard)"
                :key="folder.name"
              >
                <Card
                  :folder="folder"
                  @openFile="emit('openFile', folder.path)"
                />
              </NGi>
            </NGrid>
            <div v-if="folders.length > maxFoldersCard"
                 class="mt-4 text-center">
              <NButton
                class="w-full max-w-md"
                secondary
                size="large"
                type="primary"
                @click="maxFoldersCard += 10"
              >
                📂 Voir {{ Math.min(10, folders.length - maxFoldersCard) }}
                dossiers de plus
              </NButton>
            </div>
          </NTabPane>
          <NTabPane name="line" tab="Tableau">
            <NTable :single-line="false" class="w-full">
              <thead>
                <tr>
                  <th>Nom</th>
                  <th>Path</th>
                  <th>Création</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                <Line
                  v-for="folder in folders.slice(0, maxFoldersLine)"
                  :key="folder.name"
                  :folder="folder"
                  @openFile="emit('openFile', folder.path)"
                />
                <tr v-if="folders.length > maxFoldersLine" class="border-t border-gray-200">
                  <td colspan="3" class="text-center">
                    <NButton @click="maxFoldersLine += 10" tertiary>
                      📂 Voir {{ Math.min(10, folders.length - maxFoldersLine) }}
                      dossiers de plus
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