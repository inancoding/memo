import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { getAllWindows } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'

// label 用 editor-<id>，命中 capabilities/default.json 里已有的 "editor-*" 通配，
// 新窗口才拿得到 invoke 命令所需的权限。
export function noteWindowLabel(noteId: string) {
  return `editor-${noteId}`
}

export function noteIdFromLabel(label: string): string | null {
  return label.startsWith('editor-') ? label.slice('editor-'.length) : null
}

// 每条便签一个独立窗口；已经开着就聚焦，不重复创建。
// 窗口先建成不可见，由编辑器前端恢复好大小位置后再自己显示，避免打开瞬间跳一下。
export async function openNoteWindow(noteId: string): Promise<void> {
  const label = noteWindowLabel(noteId)
  const existing = (await getAllWindows()).find((w) => w.label === label)
  if (existing) {
    await existing.setFocus()
    return
  }

  const win = new WebviewWindow(label, {
    url: `editor.html?note=${noteId}`,
    title: 'Memo Editor',
    width: 400,
    height: 450,
    minWidth: 300,
    minHeight: 250,
    decorations: false,
    transparent: true,
    shadow: false,
    skipTaskbar: true,
    resizable: true,
    visible: false,
    dragDropEnabled: true,
  })

  await new Promise<void>((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`创建便签窗口超时: ${label}`)), 3000)
    win.once('tauri://created', () => {
      clearTimeout(timer)
      resolve()
    })
    win.once('tauri://error', (e) => {
      clearTimeout(timer)
      reject(e)
    })
  })

  // 新窗口已建成，实时记录当前开着的便签，供开机自启恢复（销毁时由 Rust 全局钩子更新）
  await invoke('record_open_notes_cmd')
}
