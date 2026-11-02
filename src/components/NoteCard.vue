<script setup lang="ts">
import { computed } from 'vue'
import type { Note } from '@/types'
import { NOTE_COLORS } from '@/types'
import { useTheme } from '@/composables/useTheme'
import { useNote } from '@/composables/useNote'

const props = defineProps<{
  note: Note
}>()

const emit = defineEmits<{
  click: [note: Note]
  pin: [note: Note]
  delete: [note: Note]
}>()

const { isDark } = useTheme()
const { formatTime } = useNote(() => props.note)

const colorStyle = computed(() => {
  const c = NOTE_COLORS[props.note.color]
  return {
    background: isDark.value ? c.dark : c.light,
    color: isDark.value ? c.darkText : c.text,
  }
})

const displayTitle = computed(() => {
  if (props.note.title) return props.note.title.slice(0, 30)
  const firstLine = props.note.content.split('\n')[0]
  return firstLine.slice(0, 30) || '新便签'
})

const preview = computed(() => {
  const lines = props.note.content.split('\n')
  return lines.slice(lines.length > 1 ? 1 : 0).join('\n').slice(0, 120)
})

const timeAgo = computed(() => formatTime(props.note.updatedAt))
</script>

<template>
  <div
    class="rounded-xl p-4 cursor-pointer flex flex-col gap-2 border border-black/5 dark:border-white/10 transition-all duration-200 hover:-translate-y-0.5 hover:shadow-md active:scale-[0.98] group"
    :style="colorStyle"
    @click="emit('click', note)"
  >
    <div class="flex items-start justify-between gap-2">
      <div class="text-sm font-semibold leading-snug line-clamp-2 flex-1">
        {{ displayTitle }}
      </div>
      <n-button
        v-if="note.isPinned"
        text
        size="tiny"
        class="flex-shrink-0 opacity-60"
      >
        <i class="i-lucide-pin w-3.5 h-3.5" />
      </n-button>
    </div>

    <div v-if="preview" class="text-xs opacity-60 leading-relaxed line-clamp-3 whitespace-pre-line flex-1">
      {{ preview }}
    </div>

    <div class="flex items-center justify-between pt-2 border-t border-black/6 dark:border-white/10 text-xs opacity-40">
      <span>{{ timeAgo }}</span>
      <div class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity">
        <n-button
          text
          size="tiny"
          title="置顶"
          @click.stop="emit('pin', note)"
        >
          <i :class="note.isPinned ? 'i-lucide-pin text-primary!' : 'i-lucide-pin-off'" class="w-3.5 h-3.5" />
        </n-button>
        <n-popconfirm @positive-click="emit('delete', note)">
          <template #trigger>
            <n-button
              text
              size="tiny"
              title="删除"
              @click.stop
            >
              <i class="i-lucide-trash-2 w-3.5 h-3.5" />
            </n-button>
          </template>
          确定删除这条便签吗？
        </n-popconfirm>
      </div>
    </div>
  </div>
</template>
