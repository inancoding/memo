<script setup lang="ts">
import NoteCard from './NoteCard.vue'
import { useI18n } from '@/composables/useI18n'
import type { Note } from '@/types'

defineProps<{
  notes: Note[]
  selectMode?: boolean
  selectedIds?: string[]
}>()

const emit = defineEmits<{
  'click-note': [note: Note]
  'pin-note': [note: Note]
  'delete-note': [note: Note]
  'toggle-select': [id: string]
}>()

const { t } = useI18n()
</script>

<template>
  <div class="grid grid-cols-1 gap-4">
    <NoteCard
      v-for="note in notes"
      :key="note.id"
      :note="note"
      :select-mode="selectMode"
      :selected="selectedIds?.includes(note.id)"
      @click="emit('click-note', note)"
      @pin="emit('pin-note', note)"
      @delete="emit('delete-note', note)"
      @toggle-select="emit('toggle-select', $event)"
    />
  </div>

  <n-empty
    v-if="notes.length === 0"
    :description="t('list.empty')"
    class="py-16"
  >
    <template #icon>
      <i class="i-lucide-sticky-note w-12 h-12 text-muted-fg opacity-40" />
    </template>
  </n-empty>
</template>
