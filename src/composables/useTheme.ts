import { ref, watch } from 'vue'

const isDark = ref(false)

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

export function useTheme() {
  loadTheme()
  applyTheme()

  watch(isDark, () => {
    localStorage.setItem('memo-theme', isDark.value ? 'dark' : 'light')
    applyTheme()
  })

  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (localStorage.getItem('memo-theme') === 'system' || !localStorage.getItem('memo-theme')) {
      isDark.value = e.matches
    }
  })

  function toggleTheme() {
    isDark.value = !isDark.value
  }

  function setTheme(mode: 'light' | 'dark' | 'system') {
    if (mode === 'system') {
      isDark.value = getSystemDark()
      localStorage.setItem('memo-theme', 'system')
    } else {
      isDark.value = mode === 'dark'
      localStorage.setItem('memo-theme', mode)
    }
  }

  return {
    isDark,
    toggleTheme,
    setTheme,
  }
}
