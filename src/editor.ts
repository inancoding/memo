import { createApp } from 'vue'
import { createPinia } from 'pinia'
import NoteEditorWindow from './components/NoteEditorWindow.vue'
import 'virtual:uno.css'
import './styles/global.css'

const app = createApp(NoteEditorWindow)
app.use(createPinia())
app.mount('#app')
