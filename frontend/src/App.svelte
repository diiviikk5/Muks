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
      appCatalog = await invoke('apps_search', { query: '', limit: 300 })
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

  function showStatus(message) {
    statusMessage = message
    setTimeout(() => {
      if (statusMessage === message) statusMessage = ''
    }, 2400)
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

    const windowPoll = setInterval(loadWindows, 2500)

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
        showStatus('Theme applied')
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
    showStatus('AI is intentionally disabled in this build.')
  }

  function handleAIRequest(query) {
    showStatus(`Saved request: ${query}`)
  }
</script>

<div class="shell-container" onclick={closeMenus} onkeydown={(event) => event.key === 'Escape' && closeMenus()} role="presentation">
  <div class="shell-backdrop" aria-hidden="true"></div>
  <div class="ambient-ring ring-a" aria-hidden="true"></div>
  <div class="ambient-ring ring-b" aria-hidden="true"></div>

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
    <div class="status-indicator">{statusMessage}</div>
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
    --bg-0: #0f1020;
    --bg-1: #1b1c33;
    --panel: rgba(31, 33, 55, 0.7);
    --panel-soft: rgba(255, 255, 255, 0.06);
    --stroke: rgba(177, 196, 255, 0.24);
    --text: #e9ecff;
    --muted: rgba(205, 214, 247, 0.68);
    --accent-cyan: #7ec8ff;
    --accent-pink: #e79ac9;
  }

  .shell-backdrop {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(180deg, rgba(18, 20, 41, 0.92), rgba(14, 15, 30, 0.95)),
      radial-gradient(circle at 88% -20%, rgba(126, 200, 255, 0.2), transparent 52%),
      radial-gradient(circle at -10% -15%, rgba(231, 154, 201, 0.18), transparent 56%);
    pointer-events: none;
  }

  .ambient-ring {
    position: absolute;
    border-radius: 999px;
    border: 1px solid rgba(169, 191, 255, 0.08);
    pointer-events: none;
  }

  .ring-a {
    width: 760px;
    height: 760px;
    left: -240px;
    top: -320px;
  }

  .ring-b {
    width: 540px;
    height: 540px;
    right: -140px;
    top: -180px;
  }

  .status-indicator {
    position: fixed;
    right: 18px;
    bottom: 106px;
    z-index: 1200;
    padding: 8px 12px;
    border-radius: 12px;
    border: 1px solid var(--stroke);
    background: var(--panel);
    color: var(--text);
    font-size: 12px;
    backdrop-filter: blur(12px) saturate(120%);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.35);
  }
</style>
