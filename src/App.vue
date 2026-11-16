<script setup lang="ts">
import { ref, computed, nextTick, onMounted, onUnmounted } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { getCurrentWindow, PhysicalPosition } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { useTheme } from './composables/useTheme'
import { useI18n, waitLanguage } from './composables/useI18n'
import { searchableText } from './composables/noteText'
import { DEFAULT_NOTE_COLOR } from './composables/noteColors'
import { useNoteStore } from './stores/notes'
import NoteList from './components/NoteList.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import WindowResizeHandles from './components/WindowResizeHandles.vue'
import type { Note } from './types'

const { isDark, mode, cycleTheme } = useTheme()
const { t } = useI18n()
const noteStore = useNoteStore()

const showSettings = ref(false)
const appWindow = getCurrentWindow()

// 自定义拖动实现，避免原生标题栏行为（右键菜单、双击最大化）
let isDragging = false
let dragStartX = 0
let dragStartY = 0
let windowStartX = 0
let windowStartY = 0

async function startDragging(e: MouseEvent) {
  // 如果点击的是按钮或其子元素，不启动拖动
  const target = e.target as HTMLElement
  if (target.closest('button') || target.closest('.n-button')) {
    return
  }
  // 只响应左键
  if (e.button !== 0) return
  
  isDragging = true
  dragStartX = e.screenX
  dragStartY = e.screenY
  const pos = await appWindow.outerPosition()
  windowStartX = pos.x
  windowStartY = pos.y
  
  document.addEventListener('mousemove', onDragMove)
  document.addEventListener('mouseup', onDragEnd)
}

function onDragMove(e: MouseEvent) {
  if (!isDragging) return
  const dx = e.screenX - dragStartX
  const dy = e.screenY - dragStartY
  appWindow.setPosition(new PhysicalPosition(windowStartX + dx, windowStartY + dy))
}

function onDragEnd() {
  isDragging = false
  document.removeEventListener('mousemove', onDragMove)
  document.removeEventListener('mouseup', onDragEnd)
}

const totalCount = computed(() => noteStore.notes.length)

// 标题栏主题按钮与「设置 - 主题模式」联动：系统 → 浅色 → 深色 循环
const themeTitle = computed(() => t('app.themeTitle', { label: t(`theme.${mode.value}`) }))
const themeIcon = computed(() => {
  if (mode.value === 'system') return 'i-lucide-monitor'
  return mode.value === 'dark' ? 'i-lucide-moon' : 'i-lucide-sun'
})

// 置顶便签排到最前，其余保持数据库返回顺序（updated_at DESC）
const sortedNotes = computed(() => {
  return [...noteStore.notes].sort((a, b) => Number(b.isPinned) - Number(a.isPinned))
})

// ===== 搜索：纯视图过滤，不改任何数据 =====
const searchOpen = ref(false)
const query = ref('')
const searchInputRef = ref<{ focus: () => void } | null>(null)

const isSearching = computed(() => searchOpen.value && query.value.trim() !== '')

const visibleNotes = computed(() => {
  const q = query.value.trim().toLowerCase()
  if (!q) return sortedNotes.value
  return sortedNotes.value.filter(note => searchableText(note.title, note.content).includes(q))
})

function toggleSearch() {
  searchOpen.value = !searchOpen.value
  if (searchOpen.value) {
    nextTick(() => searchInputRef.value?.focus())
  } else {
    query.value = ''
  }
}

function clearSearch() {
  query.value = ''
  searchInputRef.value?.focus()
}

// ===== 批量删除（多选模式）=====
const selectedIds = ref<string[]>([])
const selectMode = computed(() => selectedIds.value.length > 0)
const allSelected = computed(() =>
  visibleNotes.value.length > 0 && visibleNotes.value.every(note => selectedIds.value.includes(note.id))
)

function toggleSelect(id: string) {
  const index = selectedIds.value.indexOf(id)
  if (index === -1) selectedIds.value.push(id)
  else selectedIds.value.splice(index, 1)
}

// 全选只作用于当前可见（过滤后）的便签
function toggleSelectAll() {
  selectedIds.value = allSelected.value ? [] : visibleNotes.value.map(note => note.id)
}

async function deleteSelected() {
  const ids = [...selectedIds.value]
  for (const id of ids) {
    await noteStore.deleteNote(id)
  }
  selectedIds.value = []
}

function handleGlobalKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && searchOpen.value) {
    e.preventDefault()
    searchOpen.value = false
    query.value = ''
  }
}

// 窗口隐藏（收进托盘）后，搜索与多选状态都不保留
function handleVisibilityChange() {
  if (document.visibilityState !== 'hidden') return
  searchOpen.value = false
  query.value = ''
  selectedIds.value = []
}

let unlistenNotesChanged: UnlistenFn | null = null

const loading = ref(true)

onMounted(async () => {
  await waitLanguage()
  await noteStore.loadNotes()
  loading.value = false
  // 全局快捷键在后端新建便签后刷新列表
  unlistenNotesChanged = await listen('notes-changed', () => {
    noteStore.loadNotes()
  })
  document.addEventListener('keydown', handleGlobalKeydown)
  document.addEventListener('visibilitychange', handleVisibilityChange)
})

onUnmounted(() => {
  if (unlistenNotesChanged) unlistenNotesChanged()
  document.removeEventListener('keydown', handleGlobalKeydown)
  document.removeEventListener('visibilitychange', handleVisibilityChange)
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
    const note = await noteStore.createNote(DEFAULT_NOTE_COLOR)
    if (note) {
      console.log('Note created successfully, opening editor')
      await invoke('open_editor', { noteId: note.id })
    }
  } catch (e) {
    console.error('Failed to create note:', e)
  }
}

async function handleClickNote(note: Note) {
  // 多选模式下点击卡片只切换勾选，不打开编辑窗
  if (selectMode.value) {
    toggleSelect(note.id)
    return
  }
  
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
    <div class="h-screen flex flex-col bg-background text-foreground font-ui dark:bg-[#1C1917] dark:text-[#E7E0CC]" style="border-radius: 10px; overflow: hidden;">
      <!-- Title Bar -->
      <header class="titlebar h-10 flex items-center px-3 gap-2 bg-surface border-b border-border flex-shrink-0 select-none dark:bg-[#292524] dark:border-[#44403C]" @mousedown="startDragging">
        <!-- 多选态：顶栏整体替换为批量操作 -->
        <template v-if="selectMode">
          <div class="flex items-center gap-2 flex-1">
            <span class="text-sm font-semibold">{{ t('batch.selected', { count: selectedIds.length }) }}</span>
          </div>
          <div class="flex items-center gap-2" style="-webkit-app-region: no-drag; pointer-events: auto;">
            <n-button size="tiny" @click="toggleSelectAll">{{ t('batch.selectAll') }}</n-button>
            <n-popconfirm
              :positive-text="t('common.confirm')"
              :negative-text="t('common.cancel')"
              @positive-click="deleteSelected"
            >
              <template #trigger>
                <n-button size="tiny" type="error">{{ t('common.delete') }}</n-button>
              </template>
              {{ t('batch.deleteConfirm', { count: selectedIds.length }) }}
            </n-popconfirm>
            <n-button size="tiny" @click="selectedIds = []">{{ t('common.cancel') }}</n-button>
          </div>
        </template>

        <template v-else>
          <!-- Drag Region: Logo + Title -->
          <div class="flex items-center gap-2 flex-1 drag-region">
            <img src="/src/assets/logo.png" class="w-5 h-5 rounded-md pointer-events-none" alt="Memo" />
            <span class="text-sm font-semibold tracking-tight pointer-events-none">Memo</span>
          </div>

          <!-- Action Buttons (outside drag region) -->
          <div class="flex items-center gap-2" style="-webkit-app-region: no-drag; pointer-events: auto;">
            <n-button quaternary circle size="tiny" :title="t('app.search')" @click="toggleSearch" style="pointer-events: auto;">
              <i class="i-lucide-search w-4 h-4" />
            </n-button>
            <n-button quaternary circle size="tiny" @click="cycleTheme" :title="themeTitle" style="pointer-events: auto;">
              <i :class="themeIcon" class="w-4 h-4" />
            </n-button>
            <n-button quaternary circle size="tiny" @click="showSettings = true" :title="t('common.settings')" style="pointer-events: auto;">
              <i class="i-lucide-settings w-4 h-4" />
            </n-button>
            <n-button quaternary circle size="tiny" class="close-btn" :title="t('common.close')" @click="handleClose" style="pointer-events: auto;">
              <i class="i-lucide-x w-4 h-4" />
            </n-button>
          </div>
        </template>
      </header>

      <!-- Search Bar -->
      <div v-if="searchOpen && !selectMode" class="flex items-center gap-2 px-4 pt-3 flex-shrink-0">
        <n-input
          ref="searchInputRef"
          v-model:value="query"
          size="small"
          :placeholder="t('app.searchPlaceholder')"
          clearable
        >
          <template #prefix>
            <i class="i-lucide-search w-3.5 h-3.5 opacity-60" />
          </template>
        </n-input>
        <span class="text-xs opacity-60 whitespace-nowrap">
          {{ t('app.searchMatch', { matched: visibleNotes.length, total: totalCount }) }}
        </span>
      </div>

      <!-- Main Content -->
      <main class="flex-1 px-4 py-4 w-full overflow-y-auto">
        <div v-if="loading" class="h-full flex items-center justify-center">
          <n-spin size="large" />
        </div>
        <template v-else>
          <div class="flex items-center gap-2 mb-4">
            <h1 class="text-base font-bold">{{ t('app.allNotes') }}</h1>
            <n-tag round size="small" :bordered="false" type="warning">{{ totalCount }}</n-tag>
          </div>

          <NoteList
            v-if="visibleNotes.length > 0 || !isSearching"
            :notes="visibleNotes"
            :select-mode="selectMode"
            :selected-ids="selectedIds"
            @click-note="handleClickNote"
            @pin-note="handlePinNote"
            @delete-note="handleDeleteNote"
            @toggle-select="toggleSelect"
          />
          <n-empty v-else :description="t('app.searchEmpty')" class="py-16">
            <template #icon>
              <i class="i-lucide-search-x w-12 h-12 text-muted-fg opacity-40" />
            </template>
            <template #default>
              <n-button size="tiny" @click="clearSearch">{{ t('app.searchClear') }}</n-button>
            </template>
          </n-empty>
        </template>
      </main>

      <!-- FAB -->
      <n-button
        v-if="!selectMode"
        type="warning"
        strong
        round
        size="large"
        class="fab-btn"
        @mousedown.stop
        @click.stop="handleNewNote"
        :title="t('app.newNote')"
      >
        <template #icon>
          <i class="i-lucide-plus w-5 h-5" />
        </template>
      </n-button>

      <!-- Settings Panel -->
      <SettingsPanel v-if="showSettings" @close="showSettings = false" />
      <WindowResizeHandles />
    </div>
  </n-config-provider>
</template>

<style>
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
