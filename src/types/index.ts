export type NoteColor =
  | 'glass'
  | 'yellow'
  | 'green'
  | 'pink'
  | 'purple'
  | 'blue'
  | 'silver'
  | 'gray'

export type EdgeDirection = 'top' | 'bottom' | 'left' | 'right'

export interface Note {
  id: string
  title: string
  content: string
  color: NoteColor
  fontSize: number
  positionX: number
  positionY: number
  width: number
  height: number
  isPinned: boolean
  stickyEdge: EdgeDirection | null
  isHidden: boolean
  createdAt: string
  updatedAt: string
  deletedAt: string | null
}

export interface AppSettings {
  launchAtStartup: boolean
  edgeHide: boolean
}

export const DEFAULT_SETTINGS: AppSettings = {
  launchAtStartup: true,
  edgeHide: false,
}

export const NOTE_COLORS: Record<NoteColor, { light: string; dark: string; text: string; darkText: string }> = {
  glass:  { light: '#EAF2F8', dark: '#1B2733', text: '#123047', darkText: '#E5E7EB' },
  yellow: { light: '#FFF7D1', dark: '#4A4318', text: '#4A3B00', darkText: '#FFE66E' },
  green:  { light: '#E4F9E0', dark: '#1E3A22', text: '#14532D', darkText: '#A1EF9B' },
  pink:   { light: '#FFE4F1', dark: '#4A1E38', text: '#7A1246', darkText: '#FFAFDF' },
  purple: { light: '#F2E6FF', dark: '#2E1A47', text: '#4C1D95', darkText: '#D7AFFF' },
  blue:   { light: '#E2F1FF', dark: '#123A4A', text: '#0B4A6F', darkText: '#9EDFFF' },
  silver: { light: '#F3F2F1', dark: '#3A3A3A', text: '#1F1F1F', darkText: '#E8E8E8' },
  gray:   { light: '#696969', dark: '#4A4A4A', text: '#FFFFFF', darkText: '#FFFFFF' },
}
