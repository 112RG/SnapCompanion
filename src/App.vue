
<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window'

async function findWindow () {
  console.log('Sending command')

  setInterval(async () => {
    // Get window position and size
    const [left, right, top, bottom] = await invoke('find_window')
    // Set window position and size
    appWindow.setPosition(new PhysicalPosition(await left + 6, await top + 33))
    appWindow.setSize(new PhysicalSize(await right - await left - 12, await bottom - 40 - await top))
  }, 100)
}
</script>

<template>
  <div
    class="container"
  >
    <h1>Welcome to Tauri!</h1>

    <div class="row">
      <a
        href="https://vitejs.dev"
        target="_blank"
      >
        <img
          src="/vite.svg"
          class="logo vite"
          alt="Vite logo"
        >
      </a>
      <a
        href="https://tauri.app"
        target="_blank"
      >
        <img
          src="/tauri.svg"
          class="logo tauri"
          alt="Tauri logo"
        >
      </a>
      <a
        href="https://vuejs.org/"
        target="_blank"
      >
        <img
          src="./assets/vue.svg"
          class="logo vue"
          alt="Vue logo"
        >
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a
        href="https://code.visualstudio.com/"
        target="_blank"
      >VS Code</a>
      +
      <a
        href="https://github.com/johnsoncodehk/volar"
        target="_blank"
      >Volar</a>
      +
      <a
        href="https://github.com/tauri-apps/tauri-vscode"
        target="_blank"
      >Tauri</a>
      +
      <a
        href="https://github.com/rust-lang/rust-analyzer"
        target="_blank"
      >rust-analyzer</a>
    </p>

    <button
      type="button"
      @click="findWindow()"
    >
      Find Window
    </button>
  </div>
</template>

<style>
:root{
  background:  rgba(238, 238, 244, 0.1);
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
