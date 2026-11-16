import { ref } from 'vue'
import { emit, listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { MESSAGES, type Lang, type LangMode } from '@/i18n/messages'

// 语言模式是唯一真源（localStorage memo-lang），lang 由它解析而来，与 useTheme 同构
const mode = ref<LangMode>('system')
const lang = ref<Lang>('zh')
let syncInitialized = false

function systemLang(): Lang {
  return (navigator.language || 'en').toLowerCase().startsWith('zh') ? 'zh' : 'en'
}

function resolveMode(m: LangMode): Lang {
  return m === 'system' ? systemLang() : m
}

function readMode(): LangMode {
  const saved = localStorage.getItem('memo-lang')
  return saved === 'zh' || saved === 'en' ? saved : 'system'
}

// 托盘菜单与欢迎便签的文字在 Rust 侧生成，语言必须同步过去。
// 两个窗口都会挂载 useI18n，只让主窗口负责通知；同一次挂载只推一次，
// 因为很多组件都会调用 useI18n，逐个推会让托盘菜单反复重建。
let backendNotified = false
let languagePushed: Promise<void> = Promise.resolve()

function isMainWindow(): boolean {
  return getCurrentWindow().label === 'main'
}

function pushLanguage(): Promise<void> {
  languagePushed = invoke('set_language', { lang: lang.value })
    .then(() => undefined)
    .catch((e) => {
      console.error('set_language failed:', e)
    })
  return languagePushed
}

// 首次加载便签列表前等一次语言同步：欢迎便签是按语言改写的，
// 抢在 Rust 之前读库会先闪出另一种语言的文案。
export function waitLanguage(): Promise<void> {
  return languagePushed
}

function notifyBackend(): void {
  if (!isMainWindow() || backendNotified) return
  backendNotified = true
  void pushLanguage()
}

function initSync(): void {
  if (syncInitialized) return
  syncInitialized = true

  listen<LangMode>('language-changed', (e) => {
    mode.value = e.payload
    lang.value = resolveMode(e.payload)
  })
}

export function useI18n() {
  mode.value = readMode()
  lang.value = resolveMode(mode.value)
  initSync()
  notifyBackend()

  async function setLanguage(next: LangMode) {
    mode.value = next
    localStorage.setItem('memo-lang', next)
    lang.value = resolveMode(next)
    // 先等 Rust 换好欢迎便签，再通知其他窗口刷新
    await pushLanguage()
    emit('language-changed', next).catch(() => {})
  }

  function t(key: string, params?: Record<string, string | number>): string {
    const raw = MESSAGES[lang.value][key] ?? MESSAGES.zh[key] ?? key
    if (!params) return raw
    return raw.replace(/\{(\w+)\}/g, (_m, name: string) => String(params[name] ?? ''))
  }

  return {
    mode,
    lang,
    t,
    setLanguage,
  }
}
