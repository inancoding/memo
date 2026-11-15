import { IS_LINUX } from './usePlatform'
import { NOTE_COLORS, type NoteColor } from '@/types'

// Linux 的 WebKitGTK 在透明窗口里合成 backdrop-filter 会取到不透明白底，聚焦/失焦重绘还会越来越白，
// 所以 Linux 直接取消毛玻璃：颜色栏不出现玻璃色，已存的玻璃便签回落到颜色栏第一个实色。
export const NOTE_PALETTE: NoteColor[] = (Object.keys(NOTE_COLORS) as NoteColor[]).filter(
  (key) => !IS_LINUX || key !== 'glass',
)

export const DEFAULT_NOTE_COLOR: NoteColor = NOTE_PALETTE[0]

export function resolveNoteColor(color: NoteColor): NoteColor {
  return NOTE_PALETTE.includes(color) ? color : DEFAULT_NOTE_COLOR
}
