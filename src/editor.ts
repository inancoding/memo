import { createApp } from 'vue'
import { createPinia } from 'pinia'
import NoteEditorWindow from './components/NoteEditorWindow.vue'
import 'virtual:uno.css'
import './styles/global.css'

// 生产模式下屏蔽右键菜单
if (!import.meta.env.DEV) {
  document.addEventListener('contextmenu', (e) => {
    e.preventDefault()
  })
}

const app = createApp(NoteEditorWindow)
app.use(createPinia())
app.mount('#app')
