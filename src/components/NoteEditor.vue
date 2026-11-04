<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue'
import { MdEditor } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'
import type { Note, NoteColor } from '@/types'
import { NOTE_COLORS } from '@/types'
import { useTheme } from '@/composables/useTheme'
import { useNote } from '@/composables/useNote'

const props = defineProps<{
  note: Note
}>()

const emit = defineEmits<{
  close: []
  update: [updates: Partial<Note>]
  pin: []
}>()

const { isDark } = useTheme()
const { scheduleAutoSave, formatTime } = useNote(() => props.note)

const editContent = ref(props.note.content)

const noteBg = computed(() => {
  const c = NOTE_COLORS[props.note.color]
  return isDark.value ? c.dark : c.light
})

const noteTextColor = computed(() => {
  const c = NOTE_COLORS[props.note.color]
  return isDark.value ? c.darkText : c.text
})

const timeAgo = computed(() => formatTime(props.note.updatedAt))

const toolbarConfig = [
  'bold', 'italic', 'strike', 'underline', 'title', '-',
  'unorderedList', 'orderedList', 'task', '-',
  'fontSize', 'link', 'image', 'table', '-',
  'revoke', 'next', '=', 'preview', 'catalog',
]

watch(() => props.note.content, (val) => {
  if (val !== editContent.value) editContent.value = val
})

function applyEditorStyle() {
  const el = document.querySelector('.md-editor-custom') as HTMLElement
  if (!el) return
  const bg = noteBg.value
  const textColor = noteTextColor.value
  const selectors = [
    '', ' .md-editor-input-wrapper', ' .md-editor-editor-wrapper',
    ' .md-editor-content', ' .md-editor-scroll-wrapper',
    ' .md-editor-toolbar', ' .md-editor-toolbar-wrapper',
  ]
  selectors.forEach(sel => {
    const target = sel ? el.querySelector(sel) as HTMLElement : el
    if (target) target.style.setProperty('background', bg, 'important')
  })
  const textarea = el.querySelector('textarea') as HTMLElement
  if (textarea) {
    textarea.style.setProperty('background', bg, 'important')
    textarea.style.setProperty('color', textColor, 'important')
  }
}

watch([noteBg, noteTextColor], () => {
  nextTick(() => setTimeout(applyEditorStyle, 50))
})

onMounted(() => {
  nextTick(() => setTimeout(applyEditorStyle, 200))
})

function onModelChange(val: string) {
  editContent.value = val
  scheduleAutoSave((content) => {
    emit('update', { content })
  }, val)
}

function changeColor(color: NoteColor) {
  emit('update', { color })
}

const colorEntries = Object.entries(NOTE_COLORS) as [NoteColor, typeof NOTE_COLORS[NoteColor]][]
</script>

<template>
  <div
    class="rounded-lg shadow-lg flex flex-col overflow-hidden relative"
    :style="{ background: noteBg, color: noteTextColor, width: note.width + 'px', height: note.height + 'px', minWidth: '360px', minHeight: '300px' }"
  >
    <!-- Title Bar -->
    <div class="h-9 flex items-center px-2.5 gap-2 cursor-default flex-shrink-0 select-none" :style="{ background: noteBg }">
      <div class="flex gap-2">
        <button
          v-for="[key, val] in colorEntries"
          :key="key"
          class="w-4 h-4 rounded-full border-none cursor-pointer transition-all hover:scale-115"
          :class="{ 'ring-2 ring-primary ring-offset-1': note.color === key }"
          :style="{ background: isDark ? val.dark : val.light, boxShadow: `0 0 0 1px rgba(0,0,0,0.1)` }"
          @click="changeColor(key)"
        />
      </div>
      <div class="flex-1" />
      <n-button
        text
        size="tiny"
        :class="{ 'text-primary! opacity-100!': note.isPinned }"
        title="置顶"
        @click="emit('pin')"
      >
        <i :class="note.isPinned ? 'i-lucide-pin' : 'i-lucide-pin-off'" class="w-3.5 h-3.5" />
      </n-button>
      <n-button text size="tiny" title="关闭" @click="emit('close')">
        <i class="i-lucide-x w-3.5 h-3.5" />
      </n-button>
    </div>

    <!-- Markdown Editor -->
    <div class="flex-1 overflow-hidden" :style="{ background: noteBg }">
      <MdEditor
        :model-value="editContent"
        :theme="isDark ? 'dark' : 'light'"
        :toolbars="toolbarConfig"
        :preview="false"
        :show-code-row-number="false"
        auto-focus
        class="md-editor-custom"
        @on-change="onModelChange"
      />
    </div>

    <!-- Status Bar -->
    <div class="h-6 flex items-center justify-end px-3 text-xs opacity-40 flex-shrink-0" :style="{ background: noteBg }">
      {{ timeAgo }}
    </div>
  </div>
</template>

<style>
.md-editor-custom {
  height: 100% !important;
  border: none !important;
  border-radius: 0 !important;
  box-shadow: none !important;
}

.md-editor-custom .md-editor-toolbar {
  border-bottom: 1px solid rgba(0,0,0,0.08) !important;
}

.dark .md-editor-custom .md-editor-toolbar {
  border-bottom-color: rgba(255,255,255,0.08) !important;
}

.md-editor-custom .md-editor-toolbar-item {
  border-radius: 4px !important;
}

.md-editor-custom .md-editor-input-wrapper textarea {
  font-family: inherit !important;
}
</style>
