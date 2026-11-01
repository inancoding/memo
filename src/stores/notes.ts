import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Note, NoteColor } from '@/types'

export const useNoteStore = defineStore('notes', () => {
  const notes = ref<Note[]>([])
  const searchQuery = ref('')

  const filteredNotes = computed(() => {
    const query = searchQuery.value.toLowerCase().trim()
    const activeNotes = notes.value.filter(n => n.deletedAt === null)
    if (!query) return activeNotes
    return activeNotes.filter(
      n => n.title.toLowerCase().includes(query) || n.content.toLowerCase().includes(query)
    )
  })

  const trashedNotes = computed(() =>
    notes.value.filter(n => n.deletedAt !== null)
  )

  function createNote(color: NoteColor = 'yellow'): Note {
    const note: Note = {
      id: crypto.randomUUID(),
      title: '',
      content: '',
      color,
      fontSize: 14,
      positionX: Math.round(100 + Math.random() * 200),
      positionY: Math.round(100 + Math.random() * 200),
      width: 300,
      height: 300,
      isPinned: false,
      stickyEdge: null,
      isHidden: false,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      deletedAt: null,
    }
    notes.value.unshift(note)
    return note
  }

  function updateNote(id: string, updates: Partial<Note>) {
    const note = notes.value.find(n => n.id === id)
    if (note) {
      Object.assign(note, updates, { updatedAt: new Date().toISOString() })
      if (note.content) {
        note.title = note.content.split('\n')[0].slice(0, 50)
      }
    }
  }

  function deleteNote(id: string) {
    const note = notes.value.find(n => n.id === id)
    if (note) {
      note.deletedAt = new Date().toISOString()
    }
  }

  function restoreNote(id: string) {
    const note = notes.value.find(n => n.id === id)
    if (note) {
      note.deletedAt = null
    }
  }

  function permanentDelete(id: string) {
    const index = notes.value.findIndex(n => n.id === id)
    if (index !== -1) notes.value.splice(index, 1)
  }

  function emptyTrash() {
    notes.value = notes.value.filter(n => n.deletedAt === null)
  }

  return {
    notes,
    searchQuery,
    filteredNotes,
    trashedNotes,
    createNote,
    updateNote,
    deleteNote,
    restoreNote,
    permanentDelete,
    emptyTrash,
  }
})
