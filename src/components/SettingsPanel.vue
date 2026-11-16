<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { useSettingsStore } from '@/stores/settings'
import { useTheme } from '@/composables/useTheme'
import { useI18n } from '@/composables/useI18n'
import { IS_LINUX } from '@/composables/usePlatform'
import type { ThemeMode } from '@/composables/useTheme'
import type { LangMode } from '@/i18n/messages'

const emit = defineEmits<{ close: [] }>()

const settings = useSettingsStore()
const { mode, setTheme } = useTheme()
const { t, mode: languageMode, setLanguage } = useI18n()

const activeTab = ref<'general' | 'shortcuts' | 'about'>('general')
const appVersion = ref('')
const launchAtStartup = ref(settings.settings.launchAtStartup)
let unlistenAutostart: UnlistenFn | null = null

// 快捷键一览：只列应用自定义的三项，编辑器原生键（撤销、删除图片、续行、Esc）不在此展示
const SHORTCUTS: { keys: string; action: string; scope: string }[] = [
  { keys: 'Ctrl+Shift+N', action: 'shortcuts.newNote', scope: 'shortcuts.scopeGlobal' },
  { keys: 'Ctrl+Shift+M', action: 'shortcuts.toggleMain', scope: 'shortcuts.scopeGlobal' },
  { keys: 'Ctrl+W', action: 'shortcuts.closeWindow', scope: 'shortcuts.scopeEditor' },
]

onMounted(async () => {
  try {
    appVersion.value = await getVersion()
  } catch (e) {
    console.error('getVersion failed:', e)
  }
  // 从后端读取真实自启状态（注册表为准）
  try {
    const enabled = await invoke<boolean>('get_autostart')
    launchAtStartup.value = enabled
    settings.updateSettings({ launchAtStartup: enabled })
  } catch (e) {
    console.error('get_autostart failed:', e)
  }
  // 托盘菜单里的勾选改变自启时，这里同步显示
  unlistenAutostart = await listen<boolean>('autostart-changed', (e) => {
    launchAtStartup.value = e.payload
    settings.updateSettings({ launchAtStartup: e.payload })
  })
})

onUnmounted(() => {
  if (unlistenAutostart) unlistenAutostart()
})

async function handleAutostartChange(v: boolean) {
  launchAtStartup.value = v
  settings.updateSettings({ launchAtStartup: v })
  try {
    await invoke('set_autostart', { enabled: v })
  } catch (e) {
    console.error('set_autostart failed:', e)
  }
}

const themeMode = computed({
  get: () => mode.value,
  set: (v: ThemeMode) => setTheme(v),
})

// 语言名按各自语言书写，只有"跟随系统"跟随当前界面语言
const languageOptions = computed(() => [
  { value: 'system' as LangMode, label: t('lang.system') },
  { value: 'zh' as LangMode, label: '简体中文' },
  { value: 'en' as LangMode, label: 'English' },
])

const edgeHide = computed({
  get: () => settings.settings.edgeHide,
  set: (v: boolean) => settings.updateSettings({ edgeHide: v }),
})
</script>

<template>
  <n-modal
    :show="true"
    preset="card"
    :title="t('common.settings')"
    size="small"
    :bordered="false"
    style="max-width: 300px"
    @close="emit('close')"
    @mask-click="emit('close')"
  >
    <n-tabs v-model:value="activeTab" type="line" animated>
      <!-- General -->
      <n-tab-pane name="general" :tab="t('settings.tabGeneral')">
        <div class="flex flex-col gap-4 py-2">
          <div class="flex items-center justify-between">
            <span class="text-sm">{{ t('settings.autostart') }}</span>
            <n-switch :value="launchAtStartup" @update:value="handleAutostartChange" />
          </div>
          <div v-if="!IS_LINUX" class="flex items-center justify-between">
            <span class="text-sm">{{ t('settings.edgeHide') }}</span>
            <n-switch v-model:value="edgeHide" />
          </div>
          <div class="flex items-center justify-between gap-2">
            <span class="text-sm flex-shrink-0">{{ t('settings.language') }}</span>
            <n-select
              :value="languageMode"
              :options="languageOptions"
              size="small"
              class="w-[130px]"
              @update:value="(v: LangMode) => setLanguage(v)"
            />
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">{{ t('settings.theme') }}</span>
            <n-radio-group v-model:value="themeMode" name="themeMode" size="small">
              <n-radio-button value="light" :label="t('theme.light')" />
              <n-radio-button value="dark" :label="t('theme.dark')" />
              <n-radio-button value="system" :label="t('theme.system')" />
            </n-radio-group>
          </div>
        </div>
      </n-tab-pane>

      <!-- Shortcuts -->
      <n-tab-pane name="shortcuts" :tab="t('settings.tabShortcuts')">
        <div class="flex flex-col gap-2 py-2 max-h-[50vh] overflow-y-auto pr-1">
          <div
            v-for="item in SHORTCUTS"
            :key="item.keys"
            class="flex items-center justify-between gap-2"
          >
            <div class="flex flex-col">
              <span class="text-sm">{{ t(item.action) }}</span>
              <span class="text-xs text-muted-fg">{{ t(item.scope) }}</span>
            </div>
            <n-tag size="small" type="info">{{ item.keys }}</n-tag>
          </div>
        </div>
      </n-tab-pane>

      <!-- About -->
      <n-tab-pane name="about" :tab="t('settings.tabAbout')">
        <div class="flex flex-col gap-4 py-2">
          <div class="flex items-center gap-3 mb-2">
            <img src="/src/assets/logo.png" class="w-12 h-12 rounded-xl" alt="Memo" />
            <div>
              <div class="font-bold text-base">Memo</div>
              <div class="text-xs text-muted-fg">{{ t('settings.tagline') }}</div>
            </div>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-fg text-sm">{{ t('settings.version') }}</span>
            <span class="text-sm">v{{ appVersion || '—' }}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-fg text-sm">{{ t('settings.author') }}</span>
            <span class="text-sm">nancoding</span>
          </div>
        </div>
      </n-tab-pane>
    </n-tabs>
  </n-modal>
</template>
