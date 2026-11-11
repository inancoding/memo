import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Note, NoteColor } from '@/types'

export const useNoteStore = defineStore('notes', () => {
  const notes = ref<Note[]>([])

  async function loadNotes() {
    try {
      const result = await invoke<Note[]>('get_notes')
      notes.value = result
    } catch (e) {
      console.error('Failed to load notes:', e)
    }
  }

  async function createNote(color: NoteColor = 'yellow'): Promise<Note | null> {
    try {
      const note = await invoke<Note>('create_note', {
        request: { color, content: '' },
      })
      notes.value.unshift(note)
      return note
    } catch (e) {
      console.error('Failed to create note:', e)
      return null
    }
  }

  async function updateNote(id: string, updates: Partial<Note>) {
    try {
      const note = await invoke<Note>('update_note', {
        request: { id, ...updates },
      })
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = note
      }
    } catch (e) {
      console.error('Failed to update note:', e)
    }
  }

  async function deleteNote(id: string) {
    try {
      await invoke('delete_note', { id })
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) notes.value.splice(index, 1)
    } catch (e) {
      console.error('Failed to delete note:', e)
    }
  }

  return {
    notes,
    loadNotes,
    createNote,
    updateNote,
    deleteNote,
  }
})
