<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useTheme } from '@/composables/useTheme'
import type { Note, NoteColor } from '@/types'
import { NOTE_COLORS } from '@/types'

const { isDark } = useTheme()
const appWindow = getCurrentWindow()

const note = ref<Note | null>(null)
const isFocused = ref(false)
const editorRef = ref<HTMLDivElement | null>(null)
const noteId = ref<string | null>(null)

// 监听 open-note 事件
let unlisten: (() => void) | null = null
let unlistenClose: (() => void) | null = null
let unlistenDragDrop: (() => void) | null = null

onMounted(async () => {
  unlisten = await listen<string>('open-note', async (event) => {
    noteId.value = event.payload
    console.log('Received open-note event, noteId:', noteId.value)
    await loadNote()
  })

  // 拦截窗口关闭事件，改为隐藏
  unlistenClose = await appWindow.onCloseRequested(async (event) => {
    event.preventDefault()
    saveContent()
    await appWindow.hide()
  })

  // 监听 Tauri 窗口拖放事件
  unlistenDragDrop = await appWindow.onDragDropEvent(async (event) => {
    console.log('Drag drop event:', event)
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths
      for (const path of paths) {
        if (path.match(/\.(png|jpg|jpeg|gif|webp|bmp)$/i)) {
          const assetUrl = convertFileSrc(path)
          console.log('Inserting image:', assetUrl)
          document.execCommand('insertHTML', false, `<img src="${assetUrl}" class="editor-image" />`)
          saveContent()
        }
      }
    }
  })
})

onUnmounted(() => {
  if (unlisten) unlisten()
  if (unlistenClose) unlistenClose()
  if (unlistenDragDrop) unlistenDragDrop()
})

async function loadNote() {
  if (!noteId.value) return
  try {
    const notes = await invoke<Note[]>('get_notes')
    note.value = notes.find(n => n.id === noteId.value) || null
    if (note.value && editorRef.value) {
      await nextTick()
      editorRef.value.innerHTML = note.value.content
      applyAutoLinks()
    }
  } catch (e) {
    console.error('Failed to load note:', e)
  }
}

const noteBg = computed(() => {
  if (!note.value) return '#FEF3C7'
  const c = NOTE_COLORS[note.value.color]
  return isDark.value ? c.dark : c.light
})

const noteTextColor = computed(() => {
  if (!note.value) return '#451A03'
  const c = NOTE_COLORS[note.value.color]
  return isDark.value ? c.darkText : c.text
})

const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#D97706',
    primaryColorHover: '#B45309',
    primaryColorPressed: '#92400E',
    borderRadius: '8px',
  },
}

function handleFocus() {
  removeAutoLinks()
  isFocused.value = true
}

function handleBlur() {
  isFocused.value = false
  saveContent()
  applyAutoLinks()
}

function saveContent() {
  if (!note.value || !editorRef.value) return
  // 序列化时移除自动链接高亮，保证存库的是纯净文本
  const clone = editorRef.value.cloneNode(true) as HTMLElement
  clone.querySelectorAll('.auto-link').forEach(span => {
    const text = document.createTextNode(span.textContent || '')
    span.replaceWith(text)
  })
  clone.normalize()
  const content = clone.innerHTML
  invoke('update_note', { request: { id: note.value.id, content } }).catch(console.error)
}

// ===== 自动识别网址 / 邮箱 / 手机号并高亮 =====
const AUTO_LINK_RE =
  /(?:[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,})|(?:https?:\/\/[^\s<>"'\u0080-\uFFFF]+)|(?:www\.[^\s<>"'\u0080-\uFFFF]+)|(?:\b1[3-9]\d{9}\b)/g
const TRAILING_PUNCT_RE = /[.,;:!?)\]}。，；：！？、》」』】)]$/

function detectLinkType(s: string): 'email' | 'url' | 'phone' {
  if (s.includes('@')) return 'email'
  if (/^(https?:\/\/|www\.)/i.test(s)) return 'url'
  return 'phone'
}

function applyAutoLinks() {
  const root = editorRef.value
  if (!root) return
  const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, {
    acceptNode(node) {
      if (!node.nodeValue || !node.nodeValue.trim()) return NodeFilter.FILTER_REJECT
      const parent = node.parentElement
      if (!parent) return NodeFilter.FILTER_REJECT
      if (parent.closest('.auto-link')) return NodeFilter.FILTER_REJECT
      const tag = parent.tagName
      if (tag === 'SCRIPT' || tag === 'STYLE') return NodeFilter.FILTER_REJECT
      return NodeFilter.FILTER_ACCEPT
    },
  })
  const textNodes: Text[] = []
  let cur: Node | null
  while ((cur = walker.nextNode())) textNodes.push(cur as Text)
  textNodes.forEach(linkifyTextNode)
}

function linkifyTextNode(node: Text) {
  const text = node.nodeValue || ''
  AUTO_LINK_RE.lastIndex = 0
  let match: RegExpExecArray | null
  const frag = document.createDocumentFragment()
  let lastIndex = 0
  let found = false
  while ((match = AUTO_LINK_RE.exec(text))) {
    const raw = match[0]
    const start = match.index
    // 去掉结尾的标点，避免把句号等并入链接
    let endTrim = raw.length
    while (endTrim > 1 && TRAILING_PUNCT_RE.test(raw.slice(0, endTrim))) endTrim--
    const core = raw.slice(0, endTrim)
    const trail = raw.slice(endTrim)
    if (!core) continue
    if (start > lastIndex) frag.appendChild(document.createTextNode(text.slice(lastIndex, start)))
    const span = document.createElement('span')
    span.className = 'auto-link'
    span.setAttribute('data-type', detectLinkType(core))
    span.textContent = core
    frag.appendChild(span)
    if (trail) frag.appendChild(document.createTextNode(trail))
    lastIndex = start + raw.length
    found = true
  }
  if (!found) return
  if (lastIndex < text.length) frag.appendChild(document.createTextNode(text.slice(lastIndex)))
  node.parentNode?.replaceChild(frag, node)
}

function removeAutoLinks() {
  const root = editorRef.value
  if (!root) return
  root.querySelectorAll('.auto-link').forEach(span => {
    const text = document.createTextNode(span.textContent || '')
    span.replaceWith(text)
  })
  root.normalize()
}

function execCmd(command: string, value?: string) {
  document.execCommand(command, false, value)
  editorRef.value?.focus()
}

function insertCheckbox() {
  const checkbox = '<div class="task-item"><label class="n-checkbox-wrapper" contenteditable="false"><input type="checkbox" class="n-checkbox-input" /><span class="n-checkbox-box"><span class="n-checkbox-icon"></span></span></label><span class="task-text">待办事项</span></div><div><br></div>'
  document.execCommand('insertHTML', false, checkbox)
  editorRef.value?.focus()
}

function handleKeydown(e: KeyboardEvent) {
  // 删除选中的图片
  if ((e.key === 'Delete' || e.key === 'Backspace') && resizingImage.value) {
    e.preventDefault()
    resizingImage.value.remove()
    resizingImage.value = null
    saveContent()
    return
  }

  if (e.key === 'Enter' && !e.shiftKey) {
    const selection = window.getSelection()
    if (!selection || selection.rangeCount === 0) return
    
    const range = selection.getRangeAt(0)
    let node: Node | null = range.startContainer
    
    // 向上查找 task-item
    while (node && node !== editorRef.value) {
      if (node.nodeType === Node.ELEMENT_NODE) {
        const el = node as Element
        if (el.classList?.contains('task-item')) {
          e.preventDefault()
          
          // 检查当前 task-item 的文本是否为空（只检查纯文本，忽略 <br>）
          const textEl = el.querySelector('.task-text')
          const textContent = textEl?.textContent?.replace(/\u200B/g, '').trim() || ''
          const isEmpty = textContent === ''
          
          console.log('Enter in task-item, isEmpty:', isEmpty, 'textContent:', textContent)
          
          if (isEmpty) {
            // 空的复选框行，删除它并创建普通行
            const newDiv = document.createElement('div')
            newDiv.innerHTML = '<br>'
            el.replaceWith(newDiv)
            const newRange = document.createRange()
            newRange.setStart(newDiv, 0)
            newRange.collapse(true)
            selection.removeAllRanges()
            selection.addRange(newRange)
          } else {
            // 非空，创建新的复选框行
            const newTaskItem = document.createElement('div')
            newTaskItem.className = 'task-item'
            newTaskItem.innerHTML = '<label class="n-checkbox-wrapper" contenteditable="false"><input type="checkbox" class="n-checkbox-input" /><span class="n-checkbox-box"><span class="n-checkbox-icon"></span></span></label><span class="task-text"><br></span>'
            el.after(newTaskItem)
            // 将光标移到新行的文本区
            const newTextEl = newTaskItem.querySelector('.task-text')
            if (newTextEl) {
              const newRange = document.createRange()
              newRange.setStart(newTextEl, 0)
              newRange.collapse(true)
              selection.removeAllRanges()
              selection.addRange(newRange)
            }
          }
          saveContent()
          return
        }
      }
      node = node.parentNode
    }
  }
}

function insertImage() {
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = 'image/*'
  input.onchange = () => {
    const file = input.files?.[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = () => {
      document.execCommand('insertHTML', false, `<img src="${reader.result}" class="editor-image" />`)
    }
    reader.readAsDataURL(file)
  }
  input.click()
}

function handleDrop(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  const files = e.dataTransfer?.files
  if (!files || files.length === 0) return
  for (const file of Array.from(files)) {
    if (!file.type.startsWith('image/')) continue
    const reader = new FileReader()
    reader.onload = () => {
      document.execCommand('insertHTML', false, `<img src="${reader.result}" class="editor-image" />`)
      saveContent()
    }
    reader.readAsDataURL(file)
  }
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'copy'
  }
}

function handleDragEnter(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
}

function handleDragLeave(e: DragEvent) {
  e.preventDefault()
  e.stopPropagation()
}

// 图片点击调整大小
const resizingImage = ref<HTMLImageElement | null>(null)
const resizeHandlePos = ref({ left: '0px', top: '0px' })
let startX = 0
let startWidth = 0

function updateResizeHandlePosition() {
  if (!resizingImage.value) return
  const rect = resizingImage.value.getBoundingClientRect()
  resizeHandlePos.value = {
    left: rect.right - 6 + 'px',
    top: rect.bottom - 6 + 'px',
  }
}

const resizeHandleStyle = computed(() => {
  if (!resizingImage.value) return { display: 'none' }
  return {
    left: resizeHandlePos.value.left,
    top: resizeHandlePos.value.top,
  }
})

function handleEditorClick(e: MouseEvent) {
  const target = e.target as HTMLElement
  if (target.tagName === 'IMG') {
    document.querySelectorAll('.editor-content img.selected').forEach(img => img.classList.remove('selected'))
    target.classList.add('selected')
    resizingImage.value = target as HTMLImageElement
    nextTick(() => updateResizeHandlePosition())
  } else {
    document.querySelectorAll('.editor-content img.selected').forEach(img => img.classList.remove('selected'))
    resizingImage.value = null
  }
}

function handleResizeStart(e: MouseEvent) {
  if (!resizingImage.value) return
  e.preventDefault()
  startX = e.clientX
  startWidth = resizingImage.value.offsetWidth
  document.addEventListener('mousemove', handleResizeMove)
  document.addEventListener('mouseup', handleResizeEnd)
}

function handleResizeMove(e: MouseEvent) {
  if (!resizingImage.value) return
  const diff = e.clientX - startX
  const newWidth = Math.max(50, startWidth + diff)
  resizingImage.value.style.width = newWidth + 'px'
  // 更新手柄位置
  updateResizeHandlePosition()
}

function handleResizeEnd() {
  document.removeEventListener('mousemove', handleResizeMove)
  document.removeEventListener('mouseup', handleResizeEnd)
  saveContent()
}

function changeColor(color: NoteColor) {
  if (!note.value) return
  note.value.color = color
  invoke('update_note', { request: { id: note.value.id, color } }).catch(console.error)
}

async function handleClose() {
  saveContent()
  // 隐藏窗口而不是关闭，以便下次可以重新打开
  await appWindow.hide()
}

const colorEntries = Object.entries(NOTE_COLORS) as [NoteColor, typeof NOTE_COLORS[NoteColor]][]
</script>

<template>
  <n-config-provider
    :theme="isDark ? darkTheme : null"
    :theme-overrides="themeOverrides"
  >
    <div
      class="editor-window"
      :style="{
        background: noteBg,
        color: noteTextColor,
      }"
    >
      <!-- Title Bar / Drag Region -->
      <div class="titlebar" data-tauri-drag-region :style="{ background: noteBg + 'cc' }">
        <!-- Color Dots (visible on focus) -->
        <div class="color-dots" :class="{ 'color-dots-visible': isFocused }">
          <button
            v-for="[key, val] in colorEntries"
            :key="key"
            class="w-4 h-4 rounded-full border-none cursor-pointer transition-all hover:scale-115"
            :class="{ 'ring-2 ring-primary ring-offset-1': note?.color === key }"
            :style="{ background: isDark ? val.dark : val.light, boxShadow: '0 0 0 1px rgba(0,0,0,0.1)' }"
            @mousedown.prevent="changeColor(key)"
          />
        </div>
        <div class="flex-1" />
        <div class="close-btn-container no-drag">
          <n-button text size="tiny" title="关闭" @mousedown.prevent="handleClose">
            <i class="i-lucide-x w-4 h-4" />
          </n-button>
        </div>
      </div>

      <!-- Editor Area -->
      <div
        ref="editorRef"
        class="editor-content"
        :class="{ 'editor-focused': isFocused }"
        contenteditable="true"
        suppress-content-editable-warning
        @focus="handleFocus"
        @blur="handleBlur"
        @click="handleEditorClick"
        @keydown="handleKeydown"
        @scroll="updateResizeHandlePosition"
        @drop="handleDrop"
        @dragover="handleDragOver"
        @dragenter="handleDragEnter"
        @dragleave="handleDragLeave"
        :style="{ color: noteTextColor }"
        data-placeholder="在这里输入内容..."
      />

      <!-- Image Resize Handle -->
      <div
        v-if="resizingImage"
        class="image-resize-handle"
        :style="resizeHandleStyle"
        @mousedown="handleResizeStart"
      />

      <!-- Formatting Toolbar (visible on focus) -->
      <div class="toolbar" :class="{ 'toolbar-visible': isFocused }" :style="{ background: noteBg + 'cc', borderTop: `1px solid ${noteBg}` }">
        <n-button text size="small" title="加粗" @mousedown.prevent="execCmd('bold')">
          <i class="i-lucide-bold w-5 h-5" />
        </n-button>
        <n-button text size="small" title="斜体" @mousedown.prevent="execCmd('italic')">
          <i class="i-lucide-italic w-5 h-5" />
        </n-button>
        <n-button text size="small" title="下划线" @mousedown.prevent="execCmd('underline')">
          <i class="i-lucide-underline w-5 h-5" />
        </n-button>
        <n-button text size="small" title="删除线" @mousedown.prevent="execCmd('strikeThrough')">
          <i class="i-lucide-strikethrough w-5 h-5" />
        </n-button>
        <div class="toolbar-divider" />
        <n-button text size="small" title="无序列表" @mousedown.prevent="execCmd('insertUnorderedList')">
          <i class="i-lucide-list w-5 h-5" />
        </n-button>
        <n-button text size="small" title="有序列表" @mousedown.prevent="execCmd('insertOrderedList')">
          <i class="i-lucide-list-ordered w-5 h-5" />
        </n-button>
        <n-button text size="small" title="复选框" @mousedown.prevent="insertCheckbox">
          <i class="i-lucide-check-square w-5 h-5" />
        </n-button>
        <div class="toolbar-divider" />
        <n-button text size="small" title="插入图片" @mousedown.prevent="insertImage">
          <i class="i-lucide-image w-5 h-5" />
        </n-button>
      </div>
    </div>
  </n-config-provider>
</template>

<style>
.editor-window {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.titlebar {
  height: 36px;
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 8px;
  flex-shrink: 0;
  -webkit-app-region: drag;
}

.color-dots {
  display: flex;
  gap: 8px;
  align-items: center;
  opacity: 0;
  transform: translateY(-100%);
  transition: opacity 0.2s ease, transform 0.2s ease;
  pointer-events: none;
}

.color-dots-visible {
  opacity: 1;
  transform: translateY(0);
  pointer-events: auto;
}

.close-btn-container {
  display: flex;
  align-items: center;
  gap: 2px;
  opacity: 1;
  pointer-events: auto;
}

.no-drag,
.no-drag * {
  -webkit-app-region: no-drag;
}

.editor-content {
  flex: 1;
  padding: 12px 16px;
  overflow-y: auto;
  outline: none;
  font-size: 14px;
  line-height: 1.6;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Microsoft YaHei', sans-serif;
  word-break: break-word;
}

.editor-content b,
.editor-content strong {
  font-weight: 800;
}

.editor-content ul,
.editor-content ol {
  padding-left: 24px;
  margin: 0;
  list-style-position: outside;
}

.editor-content ul {
  list-style-type: disc;
}

.editor-content ol {
  list-style-type: decimal;
}

.editor-content li {
  margin: 0;
}

.editor-content::-webkit-scrollbar {
  width: 4px;
}

.editor-content::-webkit-scrollbar-thumb {
  background: transparent;
  border-radius: 2px;
  transition: background 0.2s ease;
}

.editor-content.editor-focused::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.15);
}

.dark .editor-content.editor-focused::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
}

.editor-content:empty::before {
  content: attr(data-placeholder);
  opacity: 0.4;
  pointer-events: none;
}

.editor-content img,
.editor-image {
  max-width: 100%;
  border-radius: 4px;
  margin: 4px 0;
  cursor: pointer;
  transition: outline 0.15s ease;
}

.editor-content img.selected,
.editor-image.selected {
  outline: 2px solid #D97706;
  outline-offset: 2px;
}

.image-resize-handle {
  position: fixed;
  width: 12px;
  height: 12px;
  background: #D97706;
  border: 2px solid white;
  border-radius: 2px;
  cursor: nwse-resize;
  z-index: 100;
  box-shadow: 0 1px 3px rgba(0,0,0,0.3);
}

.editor-content .task-item {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
  padding: 0;
  line-height: 1.6;
}

.editor-content .task-item .task-text {
  flex: 1;
  min-width: 0;
  outline: none;
  line-height: 1.6;
  display: block;
  word-break: break-word;
}

/* Naive UI style checkbox */
.n-checkbox-wrapper {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
  width: 16px;
  height: 16px;
}

.n-checkbox-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.n-checkbox-box {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  border: 1.5px solid #d9d9d9;
  border-radius: 3px;
  background: #fff;
  transition: all 0.2s ease;
}

.n-checkbox-input:checked + .n-checkbox-box {
  background: #D97706;
  border-color: #D97706;
}

.n-checkbox-input:hover + .n-checkbox-box {
  border-color: #D97706;
}

.n-checkbox-input:focus-visible + .n-checkbox-box {
  box-shadow: 0 0 0 2px rgba(217, 119, 6, 0.2);
}

.n-checkbox-icon {
  display: none;
  width: 8px;
  height: 8px;
}

.n-checkbox-input:checked + .n-checkbox-box .n-checkbox-icon {
  display: block;
  background: white;
  clip-path: polygon(14% 44%, 0 65%, 43% 100%, 100% 16%, 80% 0%, 39% 68%);
}

.dark .n-checkbox-box {
  border-color: #4a4a4a;
  background: #2a2a2a;
}

/* 自动识别的网址 / 邮箱 / 手机号高亮 */
.editor-content .auto-link {
  color: #2563EB;
  word-break: break-all;
}

.editor-content .auto-link[data-type="url"],
.editor-content .auto-link[data-type="email"] {
  text-decoration: underline;
  text-underline-offset: 2px;
}

.dark .editor-content .auto-link {
  color: #60A5FA;
}

.toolbar {
  height: 40px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  gap: 6px;
  flex-shrink: 0;
  opacity: 0;
  transform: translateY(100%);
  transition: opacity 0.2s ease, transform 0.2s ease;
  pointer-events: none;
}

.toolbar-visible {
  opacity: 1;
  transform: translateY(0);
  pointer-events: auto;
}

.toolbar-divider {
  width: 1px;
  height: 18px;
  background: rgba(0, 0, 0, 0.1);
  margin: 0 6px;
}

.dark .toolbar-divider {
  background: rgba(255, 255, 255, 0.1);
}
</style>
