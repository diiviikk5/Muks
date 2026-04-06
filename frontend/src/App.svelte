<script>
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'
  import { listen } from '@tauri-apps/api/event'
  import Taskbar from './components/Taskbar.svelte'
  import StartMenu from './components/StartMenu.svelte'
  import CommandPalette from './components/CommandPalette.svelte'

  let showStartMenu = $state(false)
  let showCommandPalette = $state(false)
  let time = $state(new Date())
  let appCatalog = $state([])
  let windows = $state([])
  let statusMessage = $state('')
  let activePreset = $state('graphite-flare')

  const presets = {
    'graphite-flare': { name: 'Graphite' },
    'forest-glass': { name: 'Forest' },
    'rose-dusk': { name: 'Rose' },
  }

  let presetName = $derived((presets[activePreset] || presets['graphite-flare']).name)

  async function updateMenuState() {
    try {
      await invoke('set_menu_open', { isOpen: showStartMenu || showCommandPalette })
    } catch {}
  }

  async function loadAppCatalog() {
    try {
      appCatalog = await invoke('apps_search', { query: '', limit: 500 })
    } catch {}
  }

  async function loadWindows() {
    try {
      windows = await invoke('windows_sync')
    } catch {}
  }

  function toast(message) {
    statusMessage = message
    setTimeout(() => {
      if (statusMessage === message) statusMessage = ''
    }, 1800)
  }

  function toggleStartMenu() {
    showStartMenu = !showStartMenu
    showCommandPalette = false
    updateMenuState()
  }

  function toggleCommandPalette() {
    showCommandPalette = !showCommandPalette
    showStartMenu = false
    updateMenuState()
  }

  function closeMenus() {
    showStartMenu = false
    showCommandPalette = false
    updateMenuState()
  }

  function setPreset(value) {
    if (!presets[value]) return
    activePreset = value
    toast(`Preset: ${presets[value].name}`)
  }

  function handleAIAction() {
    toast('AI action queue enabled.')
  }

  function handleAIRequest(query) {
    toast(`Queued: ${query}`)
  }

  onMount(() => {
    updateMenuState()
    loadAppCatalog()
    loadWindows()
    invoke('shell_subscribe').catch(() => {})

    const timer = setInterval(() => {
      time = new Date()
    }, 1000)

    const windowPoll = setInterval(loadWindows, 1800)

    const unlistenPromises = [
      listen('windows.changed', (event) => {
        windows = event.payload
      }),
      listen('apps.index.updated', () => {
        loadAppCatalog()
      }),
    ]

    const onKeyDown = (event) => {
      if (event.altKey && event.code === 'Space') {
        event.preventDefault()
        toggleCommandPalette()
      }
      if (event.key === 'Meta') {
        toggleStartMenu()
      }
      if (event.key === 'Escape') {
        closeMenus()
      }
    }

    window.addEventListener('keydown', onKeyDown)

    return () => {
      clearInterval(timer)
      clearInterval(windowPoll)
      window.removeEventListener('keydown', onKeyDown)
      Promise.all(unlistenPromises).then((list) => list.forEach((unsub) => unsub()))
    }
  })
</script>

<div class="shell" onclick={closeMenus} onkeydown={(event) => event.key === 'Escape' && closeMenus()} role="presentation">
  <Taskbar
    {time}
    apps={appCatalog}
    {windows}
    expanded={showStartMenu || showCommandPalette}
    {presetName}
    onToggleStart={toggleStartMenu}
    onToggleCommandPalette={toggleCommandPalette}
    onToggleAI={handleAIAction}
  />

  {#if showStartMenu}
    <StartMenu apps={appCatalog} windows={windows} onSelectPreset={setPreset} activePreset={activePreset} onclose={closeMenus} />
  {/if}

  {#if showCommandPalette}
    <CommandPalette apps={appCatalog} onclose={closeMenus} onExecute={handleAIRequest} />
  {/if}

  {#if statusMessage}
    <div class="toast">{statusMessage}</div>
  {/if}
</div>

<style>
  .shell {
    position: fixed;
    inset: 0;
    width: 100%;
    height: 100vh;
    overflow: hidden;
  }

  .toast {
    position: fixed;
    right: 16px;
    bottom: 88px;
    z-index: 200;
    padding: 6px 10px;
    border-radius: 8px;
    background: rgb(17 23 34 / 0.88);
    color: #f5f7fb;
    font-size: 12px;
  }
</style>
