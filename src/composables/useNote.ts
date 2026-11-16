import { ref, onUnmounted } from 'vue'
import type { Note } from '@/types'
import { useI18n } from './useI18n'

export function useNote(_note: () => Note | null) {
  const { t, lang } = useI18n()
  const saveTimer = ref<ReturnType<typeof setTimeout> | null>(null)
  const isSaving = ref(false)

  function scheduleAutoSave(callback: (content: string) => void, content: string, delay = 500) {
    if (saveTimer.value) clearTimeout(saveTimer.value)
    isSaving.value = true
    saveTimer.value = setTimeout(() => {
      callback(content)
      isSaving.value = false
    }, delay)
  }

  function formatTime(dateStr: string): string {
    const date = new Date(dateStr)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMin = Math.floor(diffMs / 60000)
    const diffHour = Math.floor(diffMs / 3600000)
    const diffDay = Math.floor(diffMs / 86400000)

    if (diffMin < 1) return t('time.justNow')
    if (diffMin < 60) return t('time.minutesAgo', { count: diffMin })
    if (diffHour < 24) return t('time.hoursAgo', { count: diffHour })
    if (diffDay < 7) return t('time.daysAgo', { count: diffDay })

    return date.toLocaleDateString(lang.value === 'zh' ? 'zh-CN' : 'en-US', { month: 'short', day: 'numeric' })
  }

  onUnmounted(() => {
    if (saveTimer.value) clearTimeout(saveTimer.value)
  })

  return {
    isSaving,
    scheduleAutoSave,
    formatTime,
  }
}
