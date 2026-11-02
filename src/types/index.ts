export type NoteColor = 'yellow' | 'blue' | 'green' | 'pink' | 'purple' | 'white'

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
  themeMode: 'light' | 'dark' | 'system'
  edgeHideEnabled: boolean
  edgeTriggerDistance: number
  edgeVisibleBarWidth: number
  edgeAnimationDuration: number
  edgeRetractDelay: number
  enabledEdges: {
    top: boolean
    bottom: boolean
    left: boolean
    right: boolean
  }
}

export const DEFAULT_SETTINGS: AppSettings = {
  launchAtStartup: false,
  themeMode: 'system',
  edgeHideEnabled: true,
  edgeTriggerDistance: 20,
  edgeVisibleBarWidth: 4,
  edgeAnimationDuration: 200,
  edgeRetractDelay: 300,
  enabledEdges: {
    top: true,
    bottom: false,
    left: true,
    right: true,
  },
}

export const NOTE_COLORS: Record<NoteColor, { light: string; dark: string; text: string; darkText: string }> = {
  yellow:  { light: '#FEF3C7', dark: '#44403C', text: '#451A03', darkText: '#FEF3C7' },
  blue:    { light: '#DBEAFE', dark: '#1E3A5F', text: '#1E3A5F', darkText: '#BFDBFE' },
  green:   { light: '#DCFCE7', dark: '#1A3A2A', text: '#14532D', darkText: '#BBF7D0' },
  pink:    { light: '#FCE7F3', dark: '#4A1E2E', text: '#831843', darkText: '#FBCFE8' },
  purple:  { light: '#F3E8FF', dark: '#2E1A47', text: '#581C87', darkText: '#E9D5FF' },
  white:   { light: '#FFFFFF', dark: '#292524', text: '#1C1917', darkText: '#E7E0CC' },
}
