<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'

type ResizeDirection = 'East' | 'North' | 'NorthEast' | 'NorthWest' | 'South' | 'SouthEast' | 'SouthWest' | 'West'

const appWindow = getCurrentWindow()

function start(dir: ResizeDirection) {
  // 关闭 shadow 后无边框窗口失去原生调整边框，改用程序化 resize
  appWindow.startResizeDragging(dir)
}
</script>

<template>
  <div class="win-resize">
    <div class="rz rz-n"  style="cursor: ns-resize"    @mousedown.prevent="start('North')" />
    <div class="rz rz-s"  style="cursor: ns-resize"    @mousedown.prevent="start('South')" />
    <div class="rz rz-w"  style="cursor: ew-resize"    @mousedown.prevent="start('West')" />
    <div class="rz rz-e"  style="cursor: ew-resize"    @mousedown.prevent="start('East')" />
    <div class="rz rz-nw" style="cursor: nwse-resize"  @mousedown.prevent="start('NorthWest')" />
    <div class="rz rz-ne" style="cursor: nesw-resize"  @mousedown.prevent="start('NorthEast')" />
    <div class="rz rz-sw" style="cursor: nesw-resize"  @mousedown.prevent="start('SouthWest')" />
    <div class="rz rz-se" style="cursor: nwse-resize"  @mousedown.prevent="start('SouthEast')" />
  </div>
</template>

<style scoped>
.win-resize {
  position: fixed;
  inset: 0;
  pointer-events: none;
  z-index: 9999;
}
.rz {
  position: absolute;
  pointer-events: auto;
}
/* 边条留出角部空间，避免与角把手重叠 */
.rz-n { top: 0; left: 12px; right: 12px; height: 6px; }
.rz-s { bottom: 0; left: 12px; right: 12px; height: 6px; }
.rz-w { left: 0; top: 12px; bottom: 12px; width: 6px; }
.rz-e { right: 0; top: 12px; bottom: 12px; width: 6px; }
/* 角把手压在边条之上 */
.rz-nw { top: 0; left: 0; width: 12px; height: 12px; z-index: 1; }
.rz-ne { top: 0; right: 0; width: 12px; height: 12px; z-index: 1; }
.rz-sw { bottom: 0; left: 0; width: 12px; height: 12px; z-index: 1; }
.rz-se { bottom: 0; right: 0; width: 12px; height: 12px; z-index: 1; }
</style>
