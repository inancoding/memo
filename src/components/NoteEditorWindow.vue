<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { darkTheme } from 'naive-ui'
import type { GlobalThemeOverrides } from 'naive-ui'
import { getCurrentWindow, currentMonitor, PhysicalSize, PhysicalPosition } from '@tauri-apps/api/window'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { emit, listen } from '@tauri-apps/api/event'
import { useTheme } from '@/composables/useTheme'
import type { Note, NoteColor } from '@/types'
import { NOTE_COLORS } from '@/types'

const { isDark } = useTheme()
const appWindow = getCurrentWindow()

const note = ref<Note | null>(null)
const isFocused = ref(false)
const editorRef = ref<HTMLDivElement | null>(null)
const noteId = ref<string | null>(null)
const isPinned = ref(false)

// 监听窗口关闭、大小变化、位置变化
let unlistenClose: (() => void) | null = null
let unlistenDragDrop: (() => void) | null = null
let unlistenResized: (() => void) | null = null
let unlistenMoved: (() => void) | null = null
let unlistenOpenNote: (() => void) | null = null

// 防抖定时器
let resizeTimer: ReturnType<typeof setTimeout> | null = null
let moveTimer: ReturnType<typeof setTimeout> | null = null

// ===== 边缘自动隐藏（贴近左/右/顶边时滑出屏幕，仅露出窄条） =====
type SnapEdge = 'left' | 'right' | 'top'
const TRIGGER_DISTANCE = 24 // 触发吸附的距离（逻辑像素）
const BAR_SIZE = 10         // 收起后露出的窄条尺寸（逻辑像素）
const ANIM_DURATION = 220   // 动画时长 ms
const REHIDE_DELAY = 700    // 鼠标离开后重新收起的延迟 ms

const snappedEdge = ref<SnapEdge | null>(null)
const isHidden = ref(false)
let isAnimating = false
// 已贴边停靠并唤回时置真：抑制"按距离检测"的自动收起（否则贴边唤回会立即被判定为靠近边缘而反复收起）。
// 仅在用户主动拖动标题栏时清除，使其可重新吸附或脱离边缘。
let suppressAutoHide = false
let revealedPos: { x: number; y: number } | null = null
let rehideTimer: ReturnType<typeof setTimeout> | null = null
let animFrame = 0

// 边缘隐藏开关（读取主窗口写入的设置，localStorage 同源跨窗口共享）
function readEdgeHideSetting(): boolean {
  try {
    const s = JSON.parse(localStorage.getItem('memo-settings') || '{}')
    return s?.edgeHide === true
  } catch {
    return false
  }
}
const edgeHideEnabled = ref(readEdgeHideSetting())
function handleStorageChange(e: StorageEvent) {
  if (e.key === 'memo-settings') edgeHideEnabled.value = readEdgeHideSetting()
}
// 关闭开关时，若当前处于收起状态则立即唤回并脱离停靠
watch(edgeHideEnabled, async (on) => {
  if (on) return
  cancelRehide()
  suppressAutoHide = false
  if (isHidden.value) await revealFromEdge()
  snappedEdge.value = null
  await appWindow.setAlwaysOnTop(isPinned.value)
})

async function readGeometry() {
  const monitor = await currentMonitor()
  if (!monitor) return null
  const size = await appWindow.outerSize()
  const pos = await appWindow.outerPosition()
  return { monitor, size, pos, scale: monitor.scaleFactor || 1 }
}

async function detectSnapEdge(): Promise<SnapEdge | null> {
  const g = await readGeometry()
  if (!g) return null
  const { monitor, size, pos, scale } = g
  const trigger = TRIGGER_DISTANCE * scale
  const mLeft = monitor.position.x
  const mTop = monitor.position.y
  const mRight = mLeft + monitor.size.width
  const dTop = pos.y - mTop
  const dLeft = pos.x - mLeft
  const dRight = mRight - (pos.x + size.width)
  let best: SnapEdge | null = null
  let bestDist = Infinity
  if (dTop <= trigger && dTop < bestDist) { best = 'top'; bestDist = dTop }
  if (dLeft <= trigger && dLeft < bestDist) { best = 'left'; bestDist = dLeft }
  if (dRight <= trigger && dRight < bestDist) { best = 'right'; bestDist = dRight }
  return best
}

async function windowBorder() {
  const outer = await appWindow.outerSize()
  const inner = await appWindow.innerSize()
  return {
    borderX: Math.max(0, Math.round((outer.width - inner.width) / 2)),
    borderY: Math.max(0, Math.round((outer.height - inner.height) / 2)),
  }
}

async function hiddenTarget(edge: SnapEdge) {
  const g = await readGeometry()
  if (!g) return null
  const { monitor, size, pos, scale } = g
  // 可调整大小的无边框窗口在 Windows 上有不可见的调整边框，outer 坐标含边框，
  // 而 DOM（把手条）在 client 区域内缩 border。必须补偿，否则露出的 client 窄条会被边框吃掉变成 0px。
  const { borderX, borderY } = await windowBorder()
  const bar = Math.round(BAR_SIZE * scale)
  const mLeft = monitor.position.x
  const mTop = monitor.position.y
  const mRight = mLeft + monitor.size.width
  // 目标：让 client 的可见窄条正好等于 bar 像素
  if (edge === 'right') return { x: Math.round(mRight - bar - borderX), y: pos.y }
  if (edge === 'left') return { x: Math.round(mLeft - size.width + bar + borderX), y: pos.y }
  return { x: pos.x, y: Math.round(mTop + bar + borderY - size.height) }
}

function animateWindow(fromX: number, fromY: number, toX: number, toY: number, onDone?: () => void) {
  cancelAnimationFrame(animFrame)
  isAnimating = true
  const start = performance.now()
  const ease = (t: number) => 1 - Math.pow(1 - t, 3)
  const step = (now: number) => {
    const t = Math.min(1, (now - start) / ANIM_DURATION)
    const e = ease(t)
    const x = Math.round(fromX + (toX - fromX) * e)
    const y = Math.round(fromY + (toY - fromY) * e)
    appWindow.setPosition(new PhysicalPosition(x, y))
    if (t < 1) {
      animFrame = requestAnimationFrame(step)
    } else {
      isAnimating = false
      onDone?.()
    }
  }
  animFrame = requestAnimationFrame(step)
}

async function hideToEdge(edge: SnapEdge) {
  const pos = await appWindow.outerPosition()
  const target = await hiddenTarget(edge)
  if (!target) return
  const g = await readGeometry()
  if (!g) return
  const { borderX, borderY } = await windowBorder()
  const mLeft = g.monitor.position.x
  const mTop = g.monitor.position.y
  const mRight = mLeft + g.monitor.size.width
  const mBottom = mTop + g.monitor.size.height
  // 唤回位置：贴靠对应边缘（client 与屏幕边齐平，视觉上"停靠"无大缝隙），
  // 另一轴保持用户拖动的位置并夹取到屏幕内，保证整窗可见。
  let rx = pos.x
  let ry = pos.y
  if (edge === 'left') rx = mLeft - borderX
  else if (edge === 'right') rx = mRight - g.size.width + borderX
  else ry = mTop - borderY
  rx = Math.min(Math.max(rx, mLeft - borderX), mRight - g.size.width + borderX)
  ry = Math.min(Math.max(ry, mTop - borderY), mBottom - g.size.height + borderY)
  revealedPos = { x: Math.round(rx), y: Math.round(ry) }
  await appWindow.setAlwaysOnTop(true)
  animateWindow(pos.x, pos.y, target.x, target.y, () => {
    isHidden.value = true
  })
}

async function revealFromEdge() {
  if (!isHidden.value || !revealedPos) return
  const pos = await appWindow.outerPosition()
  const target = revealedPos
  isHidden.value = false
  suppressAutoHide = true
  animateWindow(pos.x, pos.y, target.x, target.y)
}

async function evaluateSnap() {
  if (!edgeHideEnabled.value || isAnimating || isHidden.value || suppressAutoHide) return
  const edge = await detectSnapEdge()
  if (edge) {
    snappedEdge.value = edge
    await hideToEdge(edge)
  } else if (snappedEdge.value) {
    snappedEdge.value = null
    await appWindow.setAlwaysOnTop(isPinned.value)
  }
}

// 用户主动拖动标题栏时解除停靠抑制，使其可重新吸附或脱离边缘
function handleDragStart() {
  suppressAutoHide = false
  cancelRehide()
}

function cancelRehide() {
  if (rehideTimer) { clearTimeout(rehideTimer); rehideTimer = null }
}

function scheduleRehide() {
  if (!edgeHideEnabled.value || !snappedEdge.value || isHidden.value || isFocused.value) return
  cancelRehide()
  rehideTimer = setTimeout(async () => {
    if (isFocused.value || isHidden.value || isAnimating) return
    // 实时复检：若用户已把便签拖离边缘则脱离停靠，不再收回
    const edge = await detectSnapEdge()
    if (edge) {
      snappedEdge.value = edge
      await hideToEdge(edge)
    } else {
      snappedEdge.value = null
      suppressAutoHide = false
      await appWindow.setAlwaysOnTop(isPinned.value)
    }
  }, REHIDE_DELAY)
}

function handleWindowMouseEnter() {
  cancelRehide()
  if (edgeHideEnabled.value && isHidden.value) revealFromEdge()
}

function handleWindowMouseLeave() {
  if (isHidden.value) return
  if (edgeHideEnabled.value) scheduleRehide()
}

onMounted(async () => {
  console.log('NoteEditorWindow mounted')

  // 边缘隐藏开关：读取设置并监听主窗口的变更（同源跨窗口 storage 事件）
  edgeHideEnabled.value = readEdgeHideSetting()
  window.addEventListener('storage', handleStorageChange)

  // 监听 open-note 事件
  unlistenOpenNote = await listen<string>('open-note', async (event) => {
    console.log('Received open-note event:', event.payload)
    noteId.value = event.payload
    if (noteId.value) {
      console.log('Loading note:', noteId.value)
      await loadNote()
      console.log('Note loaded:', note.value?.id)
    }
  })
  console.log('open-note listener registered')

  // 拦截窗口关闭事件，保存位置大小后隐藏
  unlistenClose = await appWindow.onCloseRequested(async (event) => {
    event.preventDefault()
    await saveWindowGeometry()
    saveContent()
    await invoke('set_editor_visible', { visible: false }).catch(console.error)
    await appWindow.hide()
  })

  // 监听 Tauri 拖放事件（Webview 级别，v2 推荐方式）
  try {
    unlistenDragDrop = await getCurrentWebview().onDragDropEvent(async (event) => {
      console.log('Drag drop event:', event)
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths
      for (const path of paths) {
        if (path.match(/\.(png|jpg|jpeg|gif|webp|bmp)$/i)) {
          const assetUrl = convertFileSrc(path)
          console.log('Inserting image:', assetUrl)
          
          // 确保编辑器有焦点
          if (editorRef.value) {
            editorRef.value.focus()
            await nextTick()
          }
          
          // 插入图片
          if (editorRef.value) {
            const img = document.createElement('img')
            img.src = assetUrl
            img.className = 'editor-image'
            
            // 获取当前选区或追加到末尾
            const selection = window.getSelection()
            if (selection && selection.rangeCount > 0) {
              const range = selection.getRangeAt(0)
              range.deleteContents()
              range.insertNode(img)
              // 将光标移到图片后面
              range.setStartAfter(img)
              range.collapse(true)
            } else {
              editorRef.value.appendChild(img)
            }
            saveContent()
          }
        }
      }
    }
    })
    console.log('Drag drop listener registered')
  } catch (e) {
    console.error('Failed to register drag drop listener:', e)
  }

  // 监听窗口大小变化
  unlistenResized = await appWindow.onResized(async () => {
    if (resizeTimer) clearTimeout(resizeTimer)
    resizeTimer = setTimeout(async () => {
      await saveWindowGeometry()
    }, 500)
  })

  // 监听窗口位置变化
  unlistenMoved = await appWindow.onMoved(async () => {
    if (isAnimating) return
    if (moveTimer) clearTimeout(moveTimer)
    moveTimer = setTimeout(async () => {
      if (isAnimating || isHidden.value) return
      await saveWindowGeometry()
      await evaluateSnap()
    }, 500)
  })
})

onUnmounted(() => {
  if (unlistenClose) unlistenClose()
  if (unlistenDragDrop) unlistenDragDrop()
  if (unlistenResized) unlistenResized()
  if (unlistenMoved) unlistenMoved()
  if (unlistenOpenNote) unlistenOpenNote()
  if (resizeTimer) clearTimeout(resizeTimer)
  if (moveTimer) clearTimeout(moveTimer)
  if (inputTimer) clearTimeout(inputTimer)
  if (rehideTimer) clearTimeout(rehideTimer)
  cancelAnimationFrame(animFrame)
  window.removeEventListener('storage', handleStorageChange)
})

async function saveWindowGeometry() {
  if (!note.value || isAnimating) return
  try {
    const size = await appWindow.innerSize()
    let x: number
    let y: number
    if (isHidden.value && revealedPos) {
      // 收起状态露出的是窄条，持久化收起前的可见位置
      x = revealedPos.x
      y = revealedPos.y
    } else {
      const pos = await appWindow.outerPosition()
      x = pos.x
      y = pos.y
      revealedPos = { x, y }
    }
    await invoke('update_note', {
      request: {
        id: note.value.id,
        width: size.width,
        height: size.height,
        positionX: x,
        positionY: y,
      }
    })
    console.log('Saved window geometry:', { width: size.width, height: size.height, x, y })
  } catch (e) {
    console.error('Failed to save window geometry:', e)
  }
}

async function loadNote() {
  if (!noteId.value) return
  edgeHideEnabled.value = readEdgeHideSetting()
  try {
    const notes = await invoke<Note[]>('get_notes')
    note.value = notes.find(n => n.id === noteId.value) || null
    if (note.value && editorRef.value) {
      await nextTick()
      editorRef.value.innerHTML = note.value.content
      applyAutoLinks()
      
      // 恢复窗口大小和位置
      if (note.value.width && note.value.height) {
        await appWindow.setSize(new PhysicalSize(note.value.width, note.value.height))
      }
      if (note.value.positionX !== null && note.value.positionX !== undefined && 
          note.value.positionY !== null && note.value.positionY !== undefined) {
        await appWindow.setPosition(new PhysicalPosition(note.value.positionX, note.value.positionY))
        revealedPos = { x: note.value.positionX, y: note.value.positionY }
      }
      // 恢复置顶状态，与主窗口列表保持一致
      isPinned.value = note.value.isPinned
      await appWindow.setAlwaysOnTop(isPinned.value)
      console.log('Window geometry restored:', { 
        width: note.value.width, 
        height: note.value.height, 
        x: note.value.positionX, 
        y: note.value.positionY 
      })
    }
  } catch (e) {
    console.error('Failed to load note:', e)
  }
}

const isGlass = computed(() => !note.value || note.value.color === 'glass')

const noteBg = computed(() => {
  if (!note.value) return '#EAF2F8'
  const c = NOTE_COLORS[note.value.color]
  return isDark.value ? c.dark : c.light
})

const noteTextColor = computed(() => {
  if (!note.value) return '#123047'
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
  isFocused.value = true
}

function handleBlur() {
  isFocused.value = false
  refreshAutoLinks()
  saveContent()
}

// 输入时实时检测链接（防抖），并保持光标位置
let inputTimer: ReturnType<typeof setTimeout> | null = null
function handleInput() {
  if (inputTimer) clearTimeout(inputTimer)
  inputTimer = setTimeout(() => {
    refreshAutoLinks()
    saveContent()
  }, 600)
}

// 光标标记：字符偏移无法区分「上一行末尾」与「下一行开头」，
// 列表回车后按偏移还原会把光标弹回上一行末尾，改用零宽字符占位。
const CARET_MARKER = '\uFEFF'

function markCaret(root: HTMLElement): boolean {
  if (document.activeElement !== root) return false
  const sel = window.getSelection()
  if (!sel || sel.rangeCount === 0) return false
  const range = sel.getRangeAt(0)
  if (!root.contains(range.startContainer)) return false
  const el =
    range.startContainer instanceof Element
      ? range.startContainer
      : range.startContainer.parentElement
  if (el?.closest('[contenteditable="false"]')) return false
  range.insertNode(document.createTextNode(CARET_MARKER))
  return true
}

function restoreCaret(root: HTMLElement) {
  const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT)
  let node = walker.nextNode() as Text | null
  while (node) {
    const idx = node.data.indexOf(CARET_MARKER)
    if (idx >= 0) {
      node.deleteData(idx, CARET_MARKER.length)
      const sel = window.getSelection()
      if (sel) {
        const range = document.createRange()
        range.setStart(node, idx)
        range.collapse(true)
        sel.removeAllRanges()
        sel.addRange(range)
      }
      return
    }
    node = walker.nextNode() as Text | null
  }
}

// 重新检测链接：先移除旧高亮再重新应用，光标位置靠标记还原
function refreshAutoLinks() {
  const root = editorRef.value
  if (!root) return
  const marked = markCaret(root)
  removeAutoLinks()
  applyAutoLinks()
  if (marked) restoreCaret(root)
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
  invoke('update_note', { request: { id: note.value.id, content } })
    .then(() => emit('notes-changed'))
    .catch(console.error)
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

// 点击链接：网址用浏览器打开，邮箱用系统邮箱，手机号拨号
function openAutoLink(el: HTMLElement) {
  const raw = (el.textContent || '').trim()
  if (!raw) return
  const type = el.getAttribute('data-type') || detectLinkType(raw)
  let url = raw
  if (type === 'email') {
    url = `mailto:${raw}`
  } else if (type === 'phone') {
    url = `tel:${raw}`
  } else if (/^www\./i.test(raw)) {
    url = `https://${raw}`
  }
  invoke('plugin:shell|open', { path: url }).catch(err => {
    console.error('Failed to open link:', err)
  })
}

function execCmd(command: string, value?: string) {
  document.execCommand(command, false, value)
  editorRef.value?.focus()
}

function insertCheckbox() {
  const editor = editorRef.value
  if (!editor) return
  editor.focus()

  const taskItem = document.createElement('div')
  taskItem.className = 'task-item'
  taskItem.innerHTML = '<label class="n-checkbox-wrapper" contenteditable="false"><input type="checkbox" class="n-checkbox-input" /><span class="n-checkbox-box"><span class="n-checkbox-icon"></span></span></label><span class="task-text"><br></span>'

  const selection = window.getSelection()
  if (selection && selection.rangeCount > 0) {
    const range = selection.getRangeAt(0)
    range.deleteContents()
    range.insertNode(taskItem)
    // 光标定位到新复选框的文本区，不换行
    const textEl = taskItem.querySelector('.task-text')
    if (textEl) {
      const newRange = document.createRange()
      newRange.setStart(textEl, 0)
      newRange.collapse(true)
      selection.removeAllRanges()
      selection.addRange(newRange)
    }
  } else {
    editor.appendChild(taskItem)
  }
  saveContent()
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
  console.log('handleDrop called', e)
  e.preventDefault()
  e.stopPropagation()
  const files = e.dataTransfer?.files
  console.log('Files:', files)
  if (!files || files.length === 0) return
  for (const file of Array.from(files)) {
    console.log('Processing file:', file.name, file.type)
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
  // 点击自动识别的链接：打开网址 / 邮箱 / 拨号
  const linkEl = target.closest('.auto-link') as HTMLElement | null
  if (linkEl) {
    e.preventDefault()
    e.stopPropagation()
    openAutoLink(linkEl)
    return
  }
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
  invoke('update_note', { request: { id: note.value.id, color } })
    .then(() => emit('notes-changed'))
    .catch(console.error)
}

async function handleClose() {
  saveContent()
  // 隐藏窗口而不是关闭，以便下次可以重新打开
  await appWindow.hide()
}

async function togglePin() {
  const next = !isPinned.value
  try {
    // 吸附到边缘时始终保持置顶，确保窄条可见
    await appWindow.setAlwaysOnTop(next || !!snappedEdge.value)
  } catch (e) {
    console.error('setAlwaysOnTop failed:', e)
    return
  }
  isPinned.value = next
  if (!note.value) return
  invoke('update_note', { request: { id: note.value.id, isPinned: next } })
    .then(() => emit('notes-changed'))
    .catch(console.error)
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
      :class="{ 'is-glass': isGlass }"
      :style="isGlass ? undefined : { background: noteBg, color: noteTextColor }"
      @mouseenter="handleWindowMouseEnter"
      @mouseleave="handleWindowMouseLeave"
    >
      <!-- 收起后露出的实色把手条，便于看见与唤回 -->
      <div
        v-if="isHidden && snappedEdge"
        class="edge-handle"
        :class="`edge-${snappedEdge}`"
      />

      <!-- Title Bar / Drag Region -->
      <div class="titlebar" data-tauri-drag-region @mousedown="handleDragStart">
        <!-- Color Dots (visible on focus) -->
        <div class="color-dots" :class="{ 'color-dots-visible': isFocused }">
          <button
            v-for="[key, val] in colorEntries"
            :key="key"
            class="w-3.5 h-3.5 rounded-full border-none cursor-pointer transition-all hover:scale-115"
            :class="{ 'ring-2 ring-primary ring-offset-1': note?.color === key }"
            :style="key === 'glass'
              ? { background: 'rgb(255 255 255 / 0.35)', backdropFilter: 'blur(4px)', border: '1px solid rgb(255 255 255 / 0.85)', boxShadow: '0 0 0 1px rgba(0,0,0,0.1)' }
              : { background: isDark ? val.dark : val.light, boxShadow: '0 0 0 1px rgba(0,0,0,0.1)' }"
            @mousedown.prevent="changeColor(key)"
          />
        </div>
        <div class="flex-1" />
        <div class="close-btn-container no-drag">
          <n-button text size="tiny" :title="isPinned ? '取消置顶' : '固定在最上层'" @mousedown.prevent="togglePin">
            <i class="i-lucide-pin w-5 h-5" :style="{ color: isPinned ? '#D97706' : undefined }" />
          </n-button>
          <n-button text size="tiny" title="关闭" @mousedown.prevent="handleClose">
            <i class="i-lucide-x w-5 h-5" />
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
        @input="handleInput"
        @click="handleEditorClick"
        @keydown="handleKeydown"
        @scroll="updateResizeHandlePosition"
        @drop="handleDrop"
        @dragover="handleDragOver"
        @dragenter="handleDragEnter"
        @dragleave="handleDragLeave"
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
      <div class="toolbar" :class="{ 'toolbar-visible': isFocused }">
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
  position: relative;
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: 10px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

/* 毛玻璃主题（颜色栏第一项） */
.editor-window.is-glass {
  background: rgb(255 255 255 / 0.55);
  backdrop-filter: blur(32px);
  -webkit-backdrop-filter: blur(32px);
  border: 1px solid rgb(255 255 255 / 0.7);
  color: #123047;
}

.dark .editor-window.is-glass {
  background: rgb(24 24 28 / 0.55);
  border-color: rgb(255 255 255 / 0.12);
  color: #e5e7eb;
}

/* 收起后露出的实色把手条（不透明，保证可见且可 hover 唤回） */
.edge-handle {
  position: absolute;
  background: #D97706;
  z-index: 60;
  pointer-events: none;
}

/* 吸附右边：窗口右移，露出窗口左边 */
.edge-handle.edge-right {
  top: 0;
  left: 0;
  width: 8px;
  height: 100%;
}

/* 吸附左边：窗口左移，露出窗口右边 */
.edge-handle.edge-left {
  top: 0;
  right: 0;
  width: 8px;
  height: 100%;
}

/* 吸附顶边：窗口上移，露出窗口底边 */
.edge-handle.edge-top {
  bottom: 0;
  left: 0;
  width: 100%;
  height: 8px;
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
  gap: 6px;
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
  margin-left: auto;
  opacity: 1;
  pointer-events: auto;
}

.no-drag,
.no-drag * {
  -webkit-app-region: no-drag;
}

.editor-content {
  flex: 1;
  padding: 0 16px;
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

/* 列表项中的图片垂直居中 */
.editor-content li img {
  vertical-align: middle;
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

.editor-content .task-item img {
  vertical-align: middle;
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
  cursor: pointer;
  word-break: break-all;
  overflow-wrap: anywhere;
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
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 40px;
  display: flex;
  align-items: center;
  padding: 0 12px;
  gap: 6px;
  opacity: 0;
  transform: translateY(100%);
  transition: opacity 0.2s ease, transform 0.2s ease;
  pointer-events: none;
  background: inherit;
  backdrop-filter: inherit;
  -webkit-backdrop-filter: inherit;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.dark .toolbar {
  border-top-color: rgb(255 255 255 / 0.1);
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
