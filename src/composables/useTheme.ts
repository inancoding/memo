import { ref, watch } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'

const isDark = ref(false)
let syncInitialized = false

function getSystemDark(): boolean {
  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

function getMode(): string | null {
  return localStorage.getItem('memo-theme')
}

// 依据存储的模式解析出当前是否深色；system（或未设置）跟随系统
function resolveDark(): boolean {
  const mode = getMode()
  if (mode === 'dark') return true
  if (mode === 'light') return false
  return getSystemDark()
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
    const mode = getMode()
    if (mode === 'system' || !mode) {
      isDark.value = e.matches
      applyTheme()
      broadcast()
    }
  })
}

export function useTheme() {
  isDark.value = resolveDark()
  applyTheme()
  initSync()

  // 仅同步 DOM class。不要把解析后的 isDark 写回 memo-theme，
  // 否则 system 模式会被覆盖成 dark/light，导致系统切回浅色时不再跟随。
  watch(isDark, () => {
    applyTheme()
  })

  function toggleTheme() {
    isDark.value = !isDark.value
    localStorage.setItem('memo-theme', isDark.value ? 'dark' : 'light')
    applyTheme()
    broadcast()
  }

  function setTheme(mode: 'light' | 'dark' | 'system') {
    localStorage.setItem('memo-theme', mode)
    isDark.value = mode === 'system' ? getSystemDark() : mode === 'dark'
    applyTheme()
    broadcast()
  }

  return {
    isDark,
    toggleTheme,
    setTheme,
  }
}
