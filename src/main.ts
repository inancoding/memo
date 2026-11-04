import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { getCurrentWindow } from '@tauri-apps/api/window'
import App from './App.vue'
import NoteEditorWindow from './components/NoteEditorWindow.vue'
import 'virtual:uno.css'
import './styles/global.css'

// 通过窗口标签检测是否是编辑器窗口
let isEditor = false
try {
  const currentWindow = getCurrentWindow()
  const label = currentWindow.label
  isEditor = label === 'editor'
  console.log('Window label:', label, 'isEditor:', isEditor)
} catch (e) {
  console.log('Not in Tauri environment, using main App')
}

console.log('main.ts loaded, isEditor:', isEditor)

const app = createApp(isEditor ? NoteEditorWindow : App)
app.use(createPinia())
app.mount('#app')

console.log('Mounted', isEditor ? 'NoteEditorWindow' : 'App')
