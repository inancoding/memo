<script setup lang="ts">
import { ref, computed } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import { useTheme } from '@/composables/useTheme'

const emit = defineEmits<{ close: [] }>()

const settings = useSettingsStore()
const { setTheme } = useTheme()

const activeTab = ref<'general' | 'edge' | 'about'>('general')

const themeMode = computed({
  get: () => settings.settings.themeMode,
  set: (v: 'light' | 'dark' | 'system') => {
    settings.updateSettings({ themeMode: v })
    setTheme(v)
  },
})
</script>

<template>
  <n-modal
    :show="true"
    preset="card"
    title="设置"
    size="huge"
    :bordered="false"
    style="width: 560px"
    @close="emit('close')"
    @mask-click="emit('close')"
  >
    <n-tabs v-model:value="activeTab" type="line" animated>
      <!-- General -->
      <n-tab-pane name="general" tab="通用">
        <div class="flex flex-col gap-4 py-2">
          <div class="flex items-center justify-between">
            <span class="text-sm">开机自启动</span>
            <n-switch v-model:value="settings.settings.launchAtStartup" />
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
            <n-radio-group v-model:value="themeMode" name="themeMode">
              <n-radio-button value="light" label="亮色" />
              <n-radio-button value="dark" label="暗色" />
              <n-radio-button value="system" label="跟随系统" />
            </n-radio-group>
          </div>
        </div>
      </n-tab-pane>

      <!-- Edge Hide -->
      <n-tab-pane name="edge" tab="边缘隐藏">
        <div class="flex flex-col gap-4 py-2">
          <div class="flex items-center justify-between">
            <span class="text-sm">启用边缘隐藏</span>
            <n-switch v-model:value="settings.settings.edgeHideEnabled" />
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">吸附触发距离</span>
            <span class="text-xs text-muted-fg">{{ settings.settings.edgeTriggerDistance }}px</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">可见指示条宽度</span>
            <span class="text-xs text-muted-fg">{{ settings.settings.edgeVisibleBarWidth }}px</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">动画时长</span>
            <span class="text-xs text-muted-fg">{{ settings.settings.edgeAnimationDuration }}ms</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm">收回延迟</span>
            <span class="text-xs text-muted-fg">{{ settings.settings.edgeRetractDelay }}ms</span>
          </div>
          <div>
            <span class="text-sm block mb-2">启用方向</span>
            <div class="flex gap-4 flex-wrap">
              <n-checkbox
                v-for="dir in (['top', 'bottom', 'left', 'right'] as const)"
                :key="dir"
                :checked="settings.settings.enabledEdges[dir]"
                :label="dir === 'top' ? '上' : dir === 'bottom' ? '下' : dir === 'left' ? '左' : '右'"
                @update:checked="(v: boolean) => settings.updateSettings({ enabledEdges: { ...settings.settings.enabledEdges, [dir]: v } })"
              />
            </div>
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
            <span class="text-sm">v1.0.0</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-fg text-sm">技术栈</span>
            <span class="text-sm">Tauri 2 + Vue 3 + Naive UI</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-fg text-sm">许可证</span>
            <span class="text-sm">MIT</span>
          </div>
        </div>
      </n-tab-pane>
    </n-tabs>
  </n-modal>
</template>
