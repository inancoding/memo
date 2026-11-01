import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { DEFAULT_SETTINGS } from '@/types'
import type { AppSettings } from '@/types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS })

  function updateSettings(updates: Partial<AppSettings>) {
    Object.assign(settings.value, updates)
  }

  function resetSettings() {
    settings.value = { ...DEFAULT_SETTINGS }
  }

  watch(settings, (val) => {
    localStorage.setItem('memo-settings', JSON.stringify(val))
  }, { deep: true })

  const saved = localStorage.getItem('memo-settings')
  if (saved) {
    try {
      Object.assign(settings.value, JSON.parse(saved))
    } catch {
      // ignore corrupt data
    }
  }

  return {
    settings,
    updateSettings,
    resetSettings,
  }
})
