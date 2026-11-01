import { ref, onUnmounted } from 'vue'
import { useSettingsStore } from '@/stores/settings'
import type { EdgeDirection } from '@/types'

interface EdgeHideState {
  isHidden: boolean
  isAnimating: boolean
  activeEdge: EdgeDirection | null
}

export function useEdgeHide(windowEl: () => HTMLElement | null) {
  const settings = useSettingsStore()
  const state = ref<EdgeHideState>({
    isHidden: false,
    isAnimating: false,
    activeEdge: null,
  })

  let retractTimer: ReturnType<typeof setTimeout> | null = null
  let hideTimer: ReturnType<typeof setTimeout> | null = null

  function getTransform(edge: EdgeDirection): string {
    const map: Record<EdgeDirection, string> = {
      top: 'translateY(-100%)',
      bottom: 'translateY(100%)',
      left: 'translateX(-100%)',
      right: 'translateX(100%)',
    }
    return map[edge]
  }

  function hide(edge: EdgeDirection) {
    if (!settings.settings.edgeHideEnabled || state.value.isAnimating) return

    const el = windowEl()
    if (!el) return

    state.value.isAnimating = true
    state.value.activeEdge = edge

    const duration = settings.settings.edgeAnimationDuration
    el.style.transition = `transform ${duration}ms ease-in`
    el.style.transform = getTransform(edge)

    setTimeout(() => {
      state.value.isHidden = true
      state.value.isAnimating = false
    }, duration)
  }

  function show() {
    if (!state.value.isHidden || state.value.isAnimating) return

    const el = windowEl()
    if (!el) return

    state.value.isAnimating = true
    const duration = settings.settings.edgeAnimationDuration

    el.style.transition = `transform ${duration}ms ease-out`
    el.style.transform = 'translate(0, 0)'

    setTimeout(() => {
      state.value.isHidden = false
      state.value.activeEdge = null
      state.value.isAnimating = false
    }, duration)
  }

  function scheduleRetract() {
    if (retractTimer) clearTimeout(retractTimer)
    retractTimer = setTimeout(() => {
      if (state.value.activeEdge) {
        hide(state.value.activeEdge)
      }
    }, settings.settings.edgeRetractDelay)
  }

  function cancelRetract() {
    if (retractTimer) {
      clearTimeout(retractTimer)
      retractTimer = null
    }
  }

  function checkEdgeProximity(x: number, y: number, winRect: DOMRect): EdgeDirection | null {
    const threshold = settings.settings.edgeTriggerDistance
    const { enabledEdges } = settings.settings

    if (enabledEdges.top && y - winRect.top < threshold) return 'top'
    if (enabledEdges.bottom && winRect.bottom - y < threshold) return 'bottom'
    if (enabledEdges.left && x - winRect.left < threshold) return 'left'
    if (enabledEdges.right && winRect.right - x < threshold) return 'right'

    return null
  }

  function isInDetectionZone(clientX: number, clientY: number): boolean {
    const zone = 5
    const { enabledEdges } = settings.settings

    if (enabledEdges.top && clientY <= zone) return true
    if (enabledEdges.bottom && window.innerHeight - clientY <= zone) return true
    if (enabledEdges.left && clientX <= zone) return true
    if (enabledEdges.right && window.innerWidth - clientX <= zone) return true

    return false
  }

  onUnmounted(() => {
    if (retractTimer) clearTimeout(retractTimer)
    if (hideTimer) clearTimeout(hideTimer)
  })

  return {
    state,
    hide,
    show,
    scheduleRetract,
    cancelRetract,
    checkEdgeProximity,
    isInDetectionZone,
  }
}
