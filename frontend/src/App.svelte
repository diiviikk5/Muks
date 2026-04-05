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
      if (statusMessage === message) {
        statusMessage = ''
      }
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
    <div class="status-indicator">
      <span>{statusMessage}</span>
    </div>
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
    --surface: rgba(14, 20, 30, 0.78);
    --surface-soft: rgba(255, 255, 255, 0.06);
    --stroke: rgba(169, 196, 255, 0.2);
    --text: #e9f2ff;
    --muted: rgba(214, 228, 255, 0.66);
    --accent: #66a7ff;
  }

  .shell-backdrop {
    position: absolute;
    inset: 0;
    background:
      radial-gradient(circle at 20% -90%, rgba(92, 151, 255, 0.22), transparent 55%),
      radial-gradient(circle at 88% -120%, rgba(45, 198, 255, 0.14), transparent 58%),
      linear-gradient(180deg, rgba(10, 14, 22, 0.92), rgba(8, 10, 16, 0.96));
    pointer-events: none;
  }

  .status-indicator {
    position: fixed;
    right: 20px;
    bottom: 110px;
    z-index: 1200;
    padding: 8px 12px;
    border-radius: 12px;
    border: 1px solid var(--stroke);
    background: var(--surface);
    color: var(--text);
    font-size: 12px;
    backdrop-filter: blur(12px) saturate(120%);
  }
</style>
