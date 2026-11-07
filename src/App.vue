<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { useTheme } from './composables/useTheme'
import { useNoteStore } from './stores/notes'
import NoteList from './components/NoteList.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import type { Note } from './types'

const { isDark, mode, cycleTheme } = useTheme()
const noteStore = useNoteStore()

const showSettings = ref(false)
const appWindow = getCurrentWindow()

const noteCount = computed(() => noteStore.notes.filter(n => n.deletedAt === null).length)

// 标题栏主题按钮与「设置 - 主题模式」联动：系统 → 浅色 → 深色 循环
const MODE_LABELS: Record<string, string> = { light: '浅色', dark: '深色', system: '系统' }
const themeTitle = computed(() => `主题模式：${MODE_LABELS[mode.value]}（点击切换）`)
const themeIcon = computed(() => {
  if (mode.value === 'system') return 'i-lucide-monitor'
  return mode.value === 'dark' ? 'i-lucide-moon' : 'i-lucide-sun'
})

// 置顶便签排到最前，其余保持数据库返回顺序（updated_at DESC）
const visibleNotes = computed(() => {
  const active = noteStore.notes.filter(n => n.deletedAt === null)
  return [...active].sort((a, b) => Number(b.isPinned) - Number(a.isPinned))
})

let unlistenNotesChanged: UnlistenFn | null = null

const loading = ref(true)

onMounted(async () => {
  await noteStore.loadNotes()
  loading.value = false
  // 全局快捷键在后端新建便签后刷新列表
  unlistenNotesChanged = await listen('notes-changed', () => {
    noteStore.loadNotes()
  })
})

onUnmounted(() => {
  if (unlistenNotesChanged) unlistenNotesChanged()
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
    await noteStore.createNote('glass')
    console.log('Note created successfully')
  } catch (e) {
    console.error('Failed to create note:', e)
  }
}

async function handleClickNote(note: Note) {
  console.log('handleClickNote called with noteId:', note.id)
  
  try {
    await invoke('open_editor', { noteId: note.id })
    console.log('Editor window opened')
  } catch (e) {
    console.error('Failed to open editor:', e)
  }
}

function handlePinNote(note: Note) {
  noteStore.updateNote(note.id, { isPinned: !note.isPinned })
}

function handleDeleteNote(note: Note) {
  noteStore.deleteNote(note.id)
}

async function handleClose() {
  console.log('handleClose called')
  try {
    await appWindow.hide()
    console.log('Window hidden successfully')
  } catch (e) {
    console.error('Failed to hide window:', e)
  }
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
        <div class="flex items-center gap-2" style="-webkit-app-region: no-drag; pointer-events: auto;">
          <n-button quaternary circle size="tiny" @click="cycleTheme" :title="themeTitle" style="pointer-events: auto;">
            <i :class="themeIcon" class="w-4 h-4" />
          </n-button>
          <n-button quaternary circle size="tiny" @click="showSettings = true" title="设置" style="pointer-events: auto;">
            <i class="i-lucide-settings w-4 h-4" />
          </n-button>
          <n-button quaternary circle size="tiny" class="close-btn" title="关闭" @click="handleClose" style="pointer-events: auto;">
            <i class="i-lucide-x w-4 h-4" />
          </n-button>
        </div>
      </header>

      <!-- Main Content -->
      <main class="flex-1 px-4 py-4 w-full overflow-y-auto">
        <div v-if="loading" class="h-full flex items-center justify-center">
          <n-spin size="large" />
        </div>
        <template v-else>
          <div class="flex items-center gap-2 mb-4">
            <h1 class="text-base font-bold">全部便签</h1>
            <n-tag round size="small" :bordered="false" type="warning">{{ noteCount }}</n-tag>
          </div>

          <NoteList
            :notes="visibleNotes"
            @click-note="handleClickNote"
            @pin-note="handlePinNote"
            @delete-note="handleDeleteNote"
          />
        </template>
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

      <!-- Settings Panel -->
      <SettingsPanel v-if="showSettings" @close="showSettings = false" />
    </div>
  </n-config-provider>
</template>

<style>
.titlebar {
  -webkit-app-region: drag;
}

.drag-region {
  -webkit-app-region: drag;
}

.no-drag,
.no-drag * {
  -webkit-app-region: no-drag;
}

.close-btn {
  background-color: rgba(220, 38, 38, 0.15) !important;
}

.close-btn:hover {
  background-color: rgba(220, 38, 38, 0.3) !important;
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
