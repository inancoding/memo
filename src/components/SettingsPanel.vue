<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'
import { useTheme } from '@/composables/useTheme'
import type { ThemeMode } from '@/composables/useTheme'

const emit = defineEmits<{ close: [] }>()

const settings = useSettingsStore()
const { mode, setTheme } = useTheme()

const activeTab = ref<'general' | 'about'>('general')
const appVersion = ref('')
const launchAtStartup = ref(settings.settings.launchAtStartup)

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

const edgeHide = computed({
  get: () => settings.settings.edgeHide,
  set: (v: boolean) => settings.updateSettings({ edgeHide: v }),
})
</script>

<template>
  <n-modal
    :show="true"
    preset="card"
    title="设置"
    size="small"
    :bordered="false"
    style="max-width: 300px"
    @close="emit('close')"
    @mask-click="emit('close')"
  >
    <n-tabs v-model:value="activeTab" type="line" animated>
      <!-- General -->
      <n-tab-pane name="general" tab="通用">
        <div class="flex flex-col gap-4 py-2">
          <div class="flex items-center justify-between">
            <span class="text-sm">开机自启动</span>
            <n-switch :value="launchAtStartup" @update:value="handleAutostartChange" />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm">边缘自动隐藏</span>
              <span class="text-xs text-muted-fg">便签贴屏幕边缘时收起为窄条</span>
            </div>
            <n-switch v-model:value="edgeHide" />
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">新建快捷键</span>
            <n-tag size="small" type="info">Ctrl+Shift+N</n-tag>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">显示/隐藏主窗口</span>
            <n-tag size="small" type="info">Ctrl+Shift+M</n-tag>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">主题模式</span>
            <n-radio-group v-model:value="themeMode" name="themeMode" size="small">
              <n-radio-button value="light" label="浅色" />
              <n-radio-button value="dark" label="深色" />
              <n-radio-button value="system" label="系统" />
            </n-radio-group>
          </div>
        </div>
      </n-tab-pane>

      <!-- About -->
      <n-tab-pane name="about" tab="关于">
        <div class="flex flex-col gap-4 py-2">
          <div class="flex items-center gap-3 mb-2">
            <img src="/src/assets/logo.png" class="w-12 h-12 rounded-xl" alt="Memo" />
            <div>
              <div class="font-bold text-base">Memo</div>
              <div class="text-xs text-muted-fg">轻量桌面便签</div>
            </div>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-fg text-sm">版本</span>
            <span class="text-sm">v{{ appVersion || '—' }}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-fg text-sm">作者</span>
            <span class="text-sm">nancoding</span>
          </div>
        </div>
      </n-tab-pane>
    </n-tabs>
  </n-modal>
</template>
