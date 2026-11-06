import { ref, watch } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'

export type ThemeMode = 'light' | 'dark' | 'system'

// 主题模式是唯一真源，isDark 由它解析而来（system 跟随系统）
const mode = ref<ThemeMode>('system')
const isDark = ref(false)
let syncInitialized = false

function getSystemDark(): boolean {
  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

function readMode(): ThemeMode {
  const saved = localStorage.getItem('memo-theme')
  return saved === 'light' || saved === 'dark' ? saved : 'system'
}

function resolveDark(m: ThemeMode): boolean {
  return m === 'system' ? getSystemDark() : m === 'dark'
}

function applyTheme(): void {
  document.documentElement.classList.toggle('dark', isDark.value)
}

// 通知其他窗口同步主题
function broadcast(): void {
  emit('theme-changed', isDark.value).catch(() => {})
}

// 每个窗口只初始化一次跨窗口同步与系统主题监听
function initSync(): void {
  if (syncInitialized) return
  syncInitialized = true

  listen<boolean>('theme-changed', (e) => {
    if (isDark.value !== e.payload) {
      isDark.value = e.payload
      applyTheme()
    }
  })

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (mode.value === 'system') {
      isDark.value = e.matches
      applyTheme()
      broadcast()
    }
  })
}

export function useTheme() {
  mode.value = readMode()
  isDark.value = resolveDark(mode.value)
  applyTheme()
  initSync()

  // 只同步 DOM class；模式本身仅在 setTheme 中写入，
  // 否则 system 会被覆盖成 dark/light，系统切回浅色时就不再跟随。
  watch(isDark, () => {
    applyTheme()
  })

  function setTheme(next: ThemeMode) {
    mode.value = next
    localStorage.setItem('memo-theme', next)
    isDark.value = resolveDark(next)
    applyTheme()
    broadcast()
  }

  // 标题栏按钮：系统 → 浅色 → 深色 → 系统 循环切换
  function cycleTheme() {
    const order: ThemeMode[] = ['system', 'light', 'dark']
    const next = order[(order.indexOf(mode.value) + 1) % order.length]
    setTheme(next)
  }

  return {
    isDark,
    mode,
    cycleTheme,
    setTheme,
  }
}
