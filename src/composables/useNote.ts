import { ref, onUnmounted } from 'vue'
import type { Note } from '@/types'

export function useNote(_note: () => Note | null) {
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

    if (diffMin < 1) return '刚刚'
    if (diffMin < 60) return `${diffMin} 分钟前`
    if (diffHour < 24) return `${diffHour} 小时前`
    if (diffDay < 7) return `${diffDay} 天前`

    return date.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })
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
