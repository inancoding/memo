<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTheme } from './composables/useTheme'
import { useNoteStore } from './stores/notes'
import NoteList from './components/NoteList.vue'
import NoteEditor from './components/NoteEditor.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import type { Note } from './types'

const { isDark, toggleTheme } = useTheme()
const noteStore = useNoteStore()

const showSettings = ref(false)
const editingNoteId = ref<string | null>(null)
const appWindow = getCurrentWindow()

const editingNote = computed(() => {
  if (!editingNoteId.value) return null
  return noteStore.notes.find(n => n.id === editingNoteId.value) || null
})

const noteCount = computed(() => noteStore.notes.filter(n => n.deletedAt === null).length)

onMounted(() => {
  noteStore.loadNotes()
})

const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#D97706',
    primaryColorHover: '#B45309',
    primaryColorPressed: '#92400E',
    primaryColorSuppl: '#F59E0B',
    bodyColor: '#FFFBEB',
    cardColor: '#FFFFFF',
    textColorBase: '#1C1917',
    textColor1: '#1C1917',
    textColor2: '#78716C',
    textColor3: '#A8A29E',
    borderColor: '#E7E0CC',
    borderRadius: '8px',
  },
  Input: {
    borderRadius: '8px',
  },
  Button: {
    borderRadiusMedium: '8px',
  },
  Modal: {
    borderRadius: '12px',
  },
}

const darkThemeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#D97706',
    primaryColorHover: '#B45309',
    primaryColorPressed: '#92400E',
    primaryColorSuppl: '#F59E0B',
    bodyColor: '#1C1917',
    cardColor: '#292524',
    textColorBase: '#E7E0CC',
    textColor1: '#E7E0CC',
    textColor2: '#A8A29E',
    textColor3: '#78716C',
    borderColor: '#44403C',
    borderRadius: '8px',
  },
  Input: {
    borderRadius: '8px',
    color: '#292524',
    borderHoverColor: '#D97706',
    borderFocusColor: '#D97706',
    textColor: '#E7E0CC',
  },
  Button: {
    borderRadiusMedium: '8px',
  },
  Modal: {
    borderRadius: '12px',
    color: '#292524',
    textColor: '#E7E0CC',
  },
  Tabs: {
    tabTextColorActiveLine: '#D97706',
    barColor: '#D97706',
  },
  Switch: {
    railColorActive: '#D97706',
  },
  Checkbox: {
    colorChecked: '#D97706',
    borderChecked: '#D97706',
  },
  Radio: {
    buttonColorActive: '#D97706',
    textColorButtonActive: '#FFFFFF',
  },
  Tag: {
    colorWarning: '#D97706',
    textColorWarning: '#FFFFFF',
  },
  Popconfirm: {
    color: '#292524',
  },
}

async function handleNewNote() {
  console.log('handleNewNote called')
  try {
    await noteStore.createNote('yellow')
    console.log('Note created successfully')
  } catch (e) {
    console.error('Failed to create note:', e)
  }
}

function handleClickNote(note: Note) {
  editingNoteId.value = note.id
}

function handleEditorClose() {
  editingNoteId.value = null
}

function handleEditorUpdate(updates: Partial<Note>) {
  if (editingNoteId.value) {
    noteStore.updateNote(editingNoteId.value, updates)
  }
}

function handleEditorPin() {
  if (editingNoteId.value) {
    const note = noteStore.notes.find(n => n.id === editingNoteId.value)
    if (note) {
      noteStore.updateNote(editingNoteId.value, { isPinned: !note.isPinned })
    }
  }
}

function handlePinNote(note: Note) {
  noteStore.updateNote(note.id, { isPinned: !note.isPinned })
}

function handleDeleteNote(note: Note) {
  noteStore.deleteNote(note.id)
}

function handleClose() {
  console.log('handleClose called')
  appWindow.hide()
}
</script>

<template>
  <n-config-provider
    :theme="isDark ? darkTheme : null"
    :theme-overrides="isDark ? darkThemeOverrides : themeOverrides"
    class="h-screen"
  >
    <div class="h-screen flex flex-col bg-background text-foreground font-ui dark:bg-[#1C1917] dark:text-[#E7E0CC]">
      <!-- Title Bar -->
      <header class="titlebar h-10 flex items-center px-3 gap-2 bg-surface border-b border-border flex-shrink-0 select-none dark:bg-[#292524] dark:border-[#44403C]">
        <!-- Drag Region: Logo + Title -->
        <div class="flex items-center gap-2 flex-1 drag-region">
          <img src="/src/assets/logo.png" class="w-5 h-5 rounded-md pointer-events-none" alt="Memo" />
          <span class="text-sm font-semibold tracking-tight pointer-events-none">Memo</span>
        </div>

        <!-- Action Buttons (outside drag region) -->
        <div class="flex items-center gap-0.5 no-drag">
          <n-button quaternary circle size="tiny" @click="toggleTheme" :title="isDark ? '亮色模式' : '暗色模式'">
            <i :class="isDark ? 'i-lucide-moon' : 'i-lucide-sun'" class="w-4 h-4" />
          </n-button>
          <n-button quaternary circle size="tiny" @click="showSettings = true" title="设置">
            <i class="i-lucide-settings w-4 h-4" />
          </n-button>
          <n-button quaternary circle size="tiny" class="close-btn" title="关闭" @click="handleClose">
            <i class="i-lucide-x w-4 h-4" />
          </n-button>
        </div>
      </header>

      <!-- Main Content -->
      <main class="flex-1 px-4 py-4 w-full overflow-y-auto">
        <div class="flex items-center gap-2 mb-4">
          <h1 class="text-base font-bold">全部便签</h1>
          <n-tag round size="small" :bordered="false" type="warning">{{ noteCount }}</n-tag>
        </div>

        <NoteList
          :notes="noteStore.notes.filter(n => n.deletedAt === null)"
          @click-note="handleClickNote"
          @pin-note="handlePinNote"
          @delete-note="handleDeleteNote"
        />
      </main>

      <!-- FAB -->
      <n-button
        type="warning"
        strong
        round
        size="large"
        class="fab-btn"
        @mousedown.stop
        @click.stop="handleNewNote"
        title="新建便签"
      >
        <template #icon>
          <i class="i-lucide-plus w-5 h-5" />
        </template>
      </n-button>

      <!-- Note Editor -->
      <div v-if="editingNote" class="fixed inset-0 z-50 flex items-center justify-center bg-black/30 dark:bg-black/50" @click.self="handleEditorClose">
        <NoteEditor
          :note="editingNote"
          :key="editingNote.id"
          @close="handleEditorClose"
          @update="handleEditorUpdate"
          @pin="handleEditorPin"
        />
      </div>

      <!-- Settings Panel -->
      <SettingsPanel v-if="showSettings" @close="showSettings = false" />
    </div>
  </n-config-provider>
</template>

<style>
.drag-region {
  -webkit-app-region: drag;
}

.no-drag,
.no-drag * {
  -webkit-app-region: no-drag;
}

.close-btn:hover {
  background-color: rgba(220, 38, 38, 0.1) !important;
  color: #DC2626 !important;
}

.fab-btn {
  position: fixed !important;
  bottom: 20px !important;
  right: 20px !important;
  width: 48px !important;
  height: 48px !important;
  z-index: 40 !important;
}
</style>
