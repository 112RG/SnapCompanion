import { createApp } from 'vue'
import './styles.css'
import App from './App.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window'

createApp(App).mount('#app')
init()
async function init () {
  const [left, right, top, bottom] = await invoke('find_window')

  // Set window position and size
  appWindow.setPosition(new PhysicalPosition(await left + 10, await top + 36))
  console.log((await right - await left) / 2)
  appWindow.setSize(new PhysicalSize(Math.round((await right - await left) / 3.4), await bottom - 40 - await top))
}
