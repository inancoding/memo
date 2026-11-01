<script setup lang="ts">
import { ref, computed } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useTheme } from './composables/useTheme'
import { useNoteStore } from './stores/notes'
import NoteList from './components/NoteList.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import type { Note } from './types'

const { isDark, toggleTheme } = useTheme()
const noteStore = useNoteStore()

const showSettings = ref(false)
const appWindow = getCurrentWindow()

const noteCount = computed(() => noteStore.filteredNotes.length)

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

function handleNewNote() {
  noteStore.createNote('yellow')
}

function handleClickNote(note: Note) {
  console.log('Open note:', note.id)
}

function handlePinNote(note: Note) {
  noteStore.updateNote(note.id, { isPinned: !note.isPinned })
}

function handleClose() {
  appWindow.hide()
}
</script>

<template>
  <n-config-provider
    :theme="isDark ? darkTheme : null"
    :theme-overrides="themeOverrides"
    class="h-screen"
  >
    <div class="h-screen flex flex-col bg-background text-foreground font-ui">
      <!-- Title Bar (drag region + close) -->
      <header
        class="titlebar h-10 flex items-center px-3 gap-2 bg-surface border-b border-border flex-shrink-0 select-none"
        data-tauri-drag-region
      >
        <div class="flex items-center gap-2 flex-1" data-tauri-drag-region>
          <img src="/src/assets/logo.png" class="w-5 h-5 rounded-md" alt="Memo" />
          <span class="text-sm font-semibold tracking-tight">Memo</span>
        </div>

        <div class="titlebar-actions flex items-center gap-0.5">
          <n-button quaternary circle size="tiny" @click="toggleTheme" :title="isDark ? '亮色模式' : '暗色模式'">
            <i :class="isDark ? 'i-lucide-moon' : 'i-lucide-sun'" class="w-4 h-4" />
          </n-button>
          <n-button quaternary circle size="tiny" @click="showSettings = true" title="设置">
            <i class="i-lucide-settings w-4 h-4" />
          </n-button>
          <n-button quaternary circle size="tiny" class="hover:bg-destructive/10 hover:text-destructive!" title="关闭" @click="handleClose">
            <i class="i-lucide-x w-4 h-4" />
          </n-button>
        </div>
      </header>

      <!-- Search Bar -->
      <div class="px-4 py-3 bg-surface border-b border-border">
        <n-input
          v-model:value="noteStore.searchQuery"
          placeholder="搜索便签..."
          size="medium"
          round
          clearable
        >
          <template #prefix>
            <n-icon :component="undefined" class="text-muted-fg">
              <i class="i-lucide-search w-4 h-4" />
            </n-icon>
          </template>
        </n-input>
      </div>

      <!-- Main Content -->
      <main class="flex-1 px-4 py-4 w-full overflow-y-auto">
        <div class="flex items-center gap-2 mb-4">
          <h1 class="text-base font-bold">全部便签</h1>
          <n-tag round size="small" :bordered="false" type="warning">{{ noteCount }}</n-tag>
        </div>

        <NoteList
          :notes="noteStore.filteredNotes"
          @click-note="handleClickNote"
          @pin-note="handlePinNote"
        />
      </main>

      <!-- FAB -->
      <n-button
        type="warning"
        strong
        round
        size="large"
        class="fixed bottom-5 right-5 w-12 h-12 z-40"
        @click="handleNewNote"
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
.titlebar-actions,
.titlebar-actions * {
  -webkit-app-region: no-drag;
}
</style>
