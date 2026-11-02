<script setup lang="ts">
import { ref, computed, watch } from 'vue'
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
const showColorPicker = ref(false)

const colorStyle = computed(() => {
  const c = NOTE_COLORS[props.note.color]
  return {
    background: isDark.value ? c.dark : c.light,
    color: isDark.value ? c.darkText : c.text,
  }
})

const timeAgo = computed(() => formatTime(props.note.updatedAt))

watch(() => props.note.content, (val) => {
  if (val !== editContent.value) editContent.value = val
})

function onInput() {
  scheduleAutoSave((content) => {
    emit('update', { content })
  }, editContent.value)
}

function changeColor(color: NoteColor) {
  emit('update', { color })
  showColorPicker.value = false
}

const colorEntries = Object.entries(NOTE_COLORS) as [NoteColor, typeof NOTE_COLORS[NoteColor]][]
</script>

<template>
  <div
    class="rounded-lg shadow-lg flex flex-col overflow-hidden relative"
    :style="[colorStyle, { width: note.width + 'px', height: note.height + 'px', minWidth: '200px', minHeight: '150px' }]"
  >
    <!-- Title Bar -->
    <div class="h-8 flex items-center px-2 gap-1.5 cursor-default flex-shrink-0 select-none">
      <div class="flex gap-1.5">
        <button
          v-for="[key, val] in colorEntries"
          :key="key"
          class="w-2.5 h-2.5 rounded-full border-none cursor-pointer transition-transform hover:scale-120"
          :class="{ 'ring-2 ring-primary ring-offset-1': note.color === key }"
          :style="{ background: isDark ? val.dark : val.light }"
          @click="showColorPicker = !showColorPicker"
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

    <!-- Color Picker Popup -->
    <div
      v-if="showColorPicker"
      class="absolute top-8 left-2 bg-surface border border-border rounded-lg p-2.5 shadow-lg z-10"
    >
      <div class="text-xs font-semibold text-muted-fg mb-2">选择便签颜色</div>
      <div class="grid grid-cols-3 gap-1.5">
        <button
          v-for="[key, val] in colorEntries"
          :key="key"
          class="w-9 h-9 rounded-md border-2 cursor-pointer transition-all hover:scale-108"
          :class="{ 'border-primary shadow-[0_0_0_2px_rgba(217,119,6,0.3)]': note.color === key }"
          :style="{ background: isDark ? val.dark : val.light }"
          @click="changeColor(key)"
        />
      </div>
    </div>

    <!-- Editor -->
    <div class="flex-1 px-3 py-1 overflow-y-auto">
      <textarea
        v-model="editContent"
        class="w-full h-full border-none bg-transparent resize-none outline-none text-sm leading-relaxed"
        :style="{ color: colorStyle.color, fontSize: note.fontSize + 'px' }"
        placeholder="在这里输入内容..."
        @input="onInput"
      />
    </div>

    <!-- Status Bar -->
    <div class="h-6 flex items-center justify-end px-3 text-xs opacity-40 flex-shrink-0">
      {{ timeAgo }}
    </div>
  </div>
</template>
