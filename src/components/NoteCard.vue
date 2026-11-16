<script setup lang="ts">
import { computed } from 'vue'
import type { Note } from '@/types'
import { NOTE_COLORS } from '@/types'
import { resolveNoteColor } from '@/composables/noteColors'
import { htmlToText } from '@/composables/noteText'
import { useI18n } from '@/composables/useI18n'
import { useTheme } from '@/composables/useTheme'
import { useNote } from '@/composables/useNote'

const props = defineProps<{
  note: Note
  selectMode?: boolean
  selected?: boolean
}>()

const emit = defineEmits<{
  click: [note: Note]
  pin: [note: Note]
  delete: [note: Note]
  'toggle-select': [id: string]
}>()

const { t } = useI18n()
const { isDark } = useTheme()
const { formatTime } = useNote(() => props.note)

const colorStyle = computed(() => {
  const c = NOTE_COLORS[resolveNoteColor(props.note.color)]
  return {
    background: isDark.value ? c.dark : c.light,
    color: isDark.value ? c.darkText : c.text,
  }
})

// 富文本内容为 HTML，提取纯文本用于卡片预览
const contentLines = computed(() =>
  htmlToText(props.note.content).split('\n').filter(line => line.trim() !== '')
)

const displayTitle = computed(() => {
  if (props.note.title) return props.note.title.slice(0, 30)
  return (contentLines.value[0] || t('card.untitled')).slice(0, 30)
})

const preview = computed(() => contentLines.value.slice(1).join('\n').slice(0, 120))

const timeAgo = computed(() => formatTime(props.note.updatedAt))
</script>

<template>
  <div
    class="relative rounded-xl p-4 cursor-pointer flex flex-col gap-2 border border-black/5 dark:border-white/10 transition-all duration-200 hover:-translate-y-0.5 hover:shadow-md active:scale-[0.98] group"
    :style="colorStyle"
    @click="emit('click', note)"
  >
    <!-- 多选勾选框：非多选态下 hover 才出现，勾选任意一个即进入多选 -->
    <div
      class="absolute top-2 left-2 z-10 flex items-center justify-center w-5 h-5"
      :class="selectMode || selected ? '' : 'opacity-0 group-hover:opacity-100'"
      @click.stop="emit('toggle-select', note.id)"
    >
      <n-checkbox :checked="selected" />
    </div>

    <div class="flex items-start justify-between gap-2 pl-6">
      <div class="text-sm font-semibold leading-snug line-clamp-2 flex-1">
        {{ displayTitle }}
      </div>
      <n-button
        v-if="note.isPinned && !selectMode"
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
      <!-- 多选模式下禁用单卡片的置顶 / 删除 -->
      <div v-if="!selectMode" class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <n-button
          text
          size="tiny"
          :title="t('common.pin')"
          class="opacity-100!"
          @click.stop="emit('pin', note)"
        >
          <i :class="note.isPinned ? 'i-lucide-pin text-primary!' : 'i-lucide-pin-off'" class="w-4 h-4" />
        </n-button>
        <n-popconfirm
          :positive-text="t('common.confirm')"
          :negative-text="t('common.cancel')"
          @positive-click="emit('delete', note)"
        >
          <template #trigger>
            <n-button
              text
              size="tiny"
              :title="t('common.delete')"
              class="opacity-100!"
              @click.stop
            >
              <i class="i-lucide-trash-2 w-4 h-4" />
            </n-button>
          </template>
          {{ t('card.deleteConfirm') }}
        </n-popconfirm>
      </div>
    </div>
  </div>
</template>
