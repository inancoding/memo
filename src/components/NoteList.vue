<script setup lang="ts">
import NoteCard from './NoteCard.vue'
import type { Note } from '@/types'

defineProps<{
  notes: Note[]
}>()

const emit = defineEmits<{
  'click-note': [note: Note]
  'pin-note': [note: Note]
  'delete-note': [note: Note]
}>()
</script>

<template>
  <div class="grid grid-cols-1 gap-4">
    <NoteCard
      v-for="note in notes"
      :key="note.id"
      :note="note"
      @click="emit('click-note', note)"
      @pin="emit('pin-note', note)"
      @delete="emit('delete-note', note)"
    />
  </div>

  <n-empty
    v-if="notes.length === 0"
    description="还没有便签"
    class="py-16"
  >
    <template #icon>
      <i class="i-lucide-sticky-note w-12 h-12 text-muted-fg opacity-40" />
    </template>
  </n-empty>
</template>
