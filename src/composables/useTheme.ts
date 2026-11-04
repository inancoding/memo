import { ref, watch } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'

const isDark = ref(false)
let syncInitialized = false

function getSystemDark(): boolean {
  return window.matchMedia('(prefers-color-scheme: dark)').matches
}

function loadTheme(): void {
  const saved = localStorage.getItem('memo-theme')
  if (saved === 'dark') {
    isDark.value = true
  } else if (saved === 'light') {
    isDark.value = false
  } else {
    isDark.value = getSystemDark()
  }
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
    const mode = localStorage.getItem('memo-theme')
    if (mode === 'system' || !mode) {
      isDark.value = e.matches
      applyTheme()
      broadcast()
    }
  })
}

export function useTheme() {
  loadTheme()
  applyTheme()
  initSync()

  watch(isDark, () => {
    localStorage.setItem('memo-theme', isDark.value ? 'dark' : 'light')
    applyTheme()
  })

  function toggleTheme() {
    isDark.value = !isDark.value
    broadcast()
  }

  function setTheme(mode: 'light' | 'dark' | 'system') {
    if (mode === 'system') {
      isDark.value = getSystemDark()
      localStorage.setItem('memo-theme', 'system')
    } else {
      isDark.value = mode === 'dark'
      localStorage.setItem('memo-theme', mode)
    }
    broadcast()
  }

  return {
    isDark,
    toggleTheme,
    setTheme,
  }
}
