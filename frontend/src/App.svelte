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
      appCatalog = await invoke('apps_search', { query: '', limit: 360 })
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

    invoke('shell_subscribe').catch(() => {})

    const timer = setInterval(() => {
      time = new Date()
    }, 1000)

    const windowPoll = setInterval(loadWindows, 1800)

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
    ]

    return () => {
      clearInterval(timer)
      clearInterval(windowPoll)
      window.removeEventListener('keydown', handleKeydown)
      Promise.all(unlistenPromises).then((unsubs) => unsubs.forEach((u) => u()))
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
    notify('AI copilot is staged for M5 plugin rollout.')
  }

  function handleAIRequest(query) {
    notify(`Captured request: ${query}`)
  }
</script>

<div class="shell" onclick={closeMenus} onkeydown={(event) => event.key === 'Escape' && closeMenus()} role="presentation">
  <div class="atmosphere"></div>
  <div class="glow glow-left"></div>
  <div class="glow glow-right"></div>
  <div class="scan-grid"></div>

  <Taskbar
    {time}
    apps={appCatalog}
    {windows}
    onToggleStart={toggleStartMenu}
    onToggleCommandPalette={toggleCommandPalette}
    onToggleAI={handleAIAction}
  />

  {#if showStartMenu}
    <StartMenu apps={appCatalog} windows={windows} onclose={closeMenus} />
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
    font-family: 'Segoe UI Variable Display', 'Bahnschrift', 'Trebuchet MS', sans-serif;
  }

  .shell {
    position: relative;
    width: 100%;
    height: 100vh;
    overflow: hidden;
    --text-main: #eff4ff;
    --text-soft: color-mix(in oklab, #eff4ff 58%, transparent);
    --line-soft: color-mix(in oklab, #9ab9ff 22%, transparent);
    --line-strong: color-mix(in oklab, #92b2ff 42%, transparent);
    --panel-hi: color-mix(in oklab, #11172c 72%, transparent);
    --panel-lo: color-mix(in oklab, #0a1023 62%, transparent);
    --accent-cyan: oklch(0.8 0.08 236);
    --accent-gold: oklch(0.83 0.1 82);
    --accent-purple: oklch(0.72 0.09 298);
    color: var(--text-main);
  }

  .atmosphere,
  .glow,
  .scan-grid {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .atmosphere {
    background:
      radial-gradient(1300px 640px at 12% -20%, color-mix(in oklab, var(--accent-cyan) 20%, transparent), transparent 62%),
      radial-gradient(1100px 520px at 96% -20%, color-mix(in oklab, var(--accent-purple) 24%, transparent), transparent 60%),
      radial-gradient(960px 440px at 50% 120%, color-mix(in oklab, var(--accent-gold) 13%, transparent), transparent 66%),
      linear-gradient(168deg, #070b18 0%, #080d20 40%, #060916 100%);
  }

  .glow {
    opacity: 0.5;
    animation: drift 26s ease-in-out infinite;
  }

  .glow-left {
    inset: -22% auto auto -16%;
    width: 46vw;
    height: 56vh;
    background: radial-gradient(circle at 28% 36%, color-mix(in oklab, var(--accent-cyan) 36%, transparent), transparent 70%);
  }

  .glow-right {
    inset: auto -12% -30% auto;
    width: 54vw;
    height: 58vh;
    animation-delay: -8s;
    background: radial-gradient(circle at 60% 30%, color-mix(in oklab, var(--accent-purple) 34%, transparent), transparent 72%);
  }

  .scan-grid {
    opacity: 0.3;
    background-image:
      linear-gradient(90deg, color-mix(in oklab, #d2e5ff 6%, transparent) 1px, transparent 1px),
      linear-gradient(0deg, color-mix(in oklab, #d2e5ff 5%, transparent) 1px, transparent 1px);
    background-size: 96px 96px;
    mask-image: linear-gradient(180deg, transparent 0%, black 28%, black 74%, transparent 100%);
  }

  .toast {
    position: fixed;
    right: 18px;
    bottom: 100px;
    z-index: 1400;
    min-height: 34px;
    padding: 0 14px;
    border-radius: 14px;
    border: 1px solid var(--line-strong);
    background: linear-gradient(180deg, var(--panel-hi), var(--panel-lo));
    box-shadow: 0 20px 35px rgba(2, 6, 16, 0.55);
    backdrop-filter: blur(16px) saturate(135%);
    color: var(--text-main);
    font-size: 12px;
    letter-spacing: 0.03em;
    display: inline-flex;
    align-items: center;
  }

  @keyframes drift {
    0%,
    100% {
      transform: translate3d(0, 0, 0) scale(1);
    }
    50% {
      transform: translate3d(0, 2.4%, 0) scale(1.05);
    }
  }
</style>
