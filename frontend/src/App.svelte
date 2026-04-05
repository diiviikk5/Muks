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

  async function updateMenuState() {
    try {
      await invoke('set_menu_open', { isOpen: showStartMenu || showCommandPalette })
    } catch (error) {
      console.log('Failed to resize window:', error)
    }
  }

  async function loadAppCatalog() {
    try {
      appCatalog = await invoke('apps_search', { query: '', limit: 320 })
    } catch (error) {
      console.log('Failed to load app catalog:', error)
    }
  }

  async function loadWindows() {
    try {
      windows = await invoke('windows_sync')
    } catch (error) {
      console.log('Failed to load windows:', error)
    }
  }

  function notify(message) {
    statusMessage = message
    setTimeout(() => {
      if (statusMessage === message) statusMessage = ''
    }, 2200)
  }

  onMount(() => {
    updateMenuState()
    loadAppCatalog()
    loadWindows()

    invoke('shell_subscribe').catch((error) => {
      console.log('Subscribe command failed:', error)
    })

    const timer = setInterval(() => {
      time = new Date()
    }, 1000)

    const windowPoll = setInterval(loadWindows, 2200)

    const handleKeydown = (event) => {
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

    window.addEventListener('keydown', handleKeydown)

    const unlistenPromises = [
      listen('windows.changed', (event) => {
        windows = event.payload
      }),
      listen('apps.index.updated', () => {
        loadAppCatalog()
      }),
      listen('theme.changed', () => {
        notify('Theme applied')
      }),
    ]

    return () => {
      clearInterval(timer)
      clearInterval(windowPoll)
      window.removeEventListener('keydown', handleKeydown)
      Promise.all(unlistenPromises).then((unsubscribers) => {
        unsubscribers.forEach((unsub) => unsub())
      })
    }
  })

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

  function handleAIAction() {
    notify('AI module is disabled in this build.')
  }

  function handleAIRequest(query) {
    notify(`Saved request: ${query}`)
  }
</script>

<div class="shell-container" onclick={closeMenus} onkeydown={(event) => event.key === 'Escape' && closeMenus()} role="presentation">
  <div class="backdrop"></div>
  <div class="halo halo-a"></div>
  <div class="halo halo-b"></div>
  <div class="scanline"></div>

  <Taskbar
    {time}
    apps={appCatalog}
    {windows}
    onToggleStart={toggleStartMenu}
    onToggleCommandPalette={toggleCommandPalette}
    onToggleAI={handleAIAction}
  />

  {#if showStartMenu}
    <StartMenu apps={appCatalog} onclose={closeMenus} />
  {/if}

  {#if showCommandPalette}
    <CommandPalette apps={appCatalog} onclose={closeMenus} onExecute={handleAIRequest} />
  {/if}

  {#if statusMessage}
    <div class="toast">{statusMessage}</div>
  {/if}
</div>

<style>
  :global(html),
  :global(body) {
    background: transparent;
  }

  .shell-container {
    position: relative;
    width: 100%;
    height: 100vh;
    overflow: hidden;
    --bg-0: #0f1122;
    --bg-1: #171a32;
    --panel: rgba(27, 31, 55, 0.74);
    --stroke: rgba(170, 193, 255, 0.22);
    --text: #eaf0ff;
    --muted: rgba(203, 215, 246, 0.7);
    --accent: #7fc8ff;
    --accent-soft: rgba(127, 200, 255, 0.25);
    --pink: #e6a8d3;
  }

  .backdrop,
  .halo,
  .scanline {
    position: absolute;
    pointer-events: none;
  }

  .backdrop {
    inset: 0;
    background:
      radial-gradient(circle at 12% -30%, rgba(127, 200, 255, 0.24), transparent 46%),
      radial-gradient(circle at 88% -26%, rgba(230, 168, 211, 0.2), transparent 52%),
      linear-gradient(180deg, rgba(15, 17, 34, 0.94), rgba(12, 13, 27, 0.96));
  }

  .halo {
    border: 1px solid rgba(173, 194, 255, 0.08);
    border-radius: 999px;
  }

  .halo-a {
    width: 760px;
    height: 760px;
    left: -210px;
    top: -340px;
  }

  .halo-b {
    width: 560px;
    height: 560px;
    right: -130px;
    top: -210px;
  }

  .scanline {
    inset: 0;
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.015) 0, rgba(255, 255, 255, 0) 20%);
  }

  .toast {
    position: fixed;
    right: 18px;
    bottom: 104px;
    z-index: 1300;
    padding: 8px 12px;
    border-radius: 12px;
    border: 1px solid var(--stroke);
    background: var(--panel);
    color: var(--text);
    font-size: 12px;
    backdrop-filter: blur(12px) saturate(120%);
    box-shadow: 0 10px 24px rgba(0, 0, 0, 0.4);
  }
</style>
