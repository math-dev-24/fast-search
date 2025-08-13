<script setup lang="ts">
import { NButton, NCard, NDatePicker, NIcon, NSpace } from 'naive-ui';
import { CalendarOutline } from '@vicons/ionicons5';
import { useSearchStore } from '../../shared';
import { useDate } from '../../composables/useDate';
import { DateMode } from '../../types';

const emit = defineEmits<{ (e: 'handleSearch'): void }>();

const searchStore = useSearchStore();
const { dateShortcuts, listShortcuts, isSelectedDate } = useDate();

const handleDateChange = () => {
  if (searchStore.auto_submit) emit('handleSearch');
};

const getDateShortcuts = (shortcut: string) => {
  return dateShortcuts[shortcut as keyof typeof dateShortcuts]();
};

const updateDateRange = (shortcut: string) => {
  searchStore.query.filters.date_range = getDateShortcuts(shortcut);
  handleDateChange();
};

const updateDateMode = () => {
  searchStore.query.filters.date_mode =
    searchStore.query.filters.date_mode === DateMode.CREATE ? DateMode.MODIFY : DateMode.CREATE;
  handleDateChange();
};
</script>

<template>
  <NCard class="filter-card date-card" size="small">
    <template #header>
      <div class="filter-header">
        <NIcon class="text-green-500" size="16">
          <CalendarOutline />
        </NIcon>
        <span>
          Date de
          <NButton class="date-mode-btn" text type="primary" @click="updateDateMode">
            {{ searchStore.query.filters.date_mode === DateMode.CREATE ? 'création' : 'modification' }}
          </NButton>
        </span>
      </div>
    </template>
    <NSpace size="small" vertical>
      <NDatePicker
        v-model:value="searchStore.query.filters.date_range"
        clearable
        placeholder="Sélectionner une période"
        type="daterange"
        @update:value="handleDateChange"
      />
      <NSpace class="date-shortcuts" size="small">
        <NButton
          v-for="shortcut in listShortcuts"
          :key="shortcut"
          :disabled="isSelectedDate(searchStore.query.filters.date_range, getDateShortcuts(shortcut))"
          :type="isSelectedDate(searchStore.query.filters.date_range, getDateShortcuts(shortcut)) ? 'primary' : 'default'"
          secondary
          size="tiny"
          @click="() => { updateDateRange(shortcut) }"
        >
          {{ shortcut }}
        </NButton>
      </NSpace>
    </NSpace>
  </NCard>
</template>
