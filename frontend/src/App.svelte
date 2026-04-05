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

  const uiPresets = {
    'graphite-flare': {
      name: 'Graphite Flare',
      accentA: 'oklch(0.76 0.1 58)',
      accentB: 'oklch(0.72 0.05 250)',
      accentC: 'oklch(0.78 0.07 25)',
      baseTop: '#0f1117',
      baseBottom: '#07080d',
    },
    'forest-glass': {
      name: 'Forest Glass',
      accentA: 'oklch(0.76 0.08 160)',
      accentB: 'oklch(0.7 0.06 215)',
      accentC: 'oklch(0.8 0.09 90)',
      baseTop: '#0b1512',
      baseBottom: '#070d0b',
    },
    'rose-dusk': {
      name: 'Rose Dusk',
      accentA: 'oklch(0.78 0.09 20)',
      accentB: 'oklch(0.74 0.08 315)',
      accentC: 'oklch(0.8 0.07 75)',
      baseTop: '#15101a',
      baseBottom: '#0c0910',
    },
  }

  let preset = $derived(uiPresets[activePreset] ?? uiPresets['graphite-flare'])
  let themeStyle = $derived(`
    --accent-a: ${preset.accentA};
    --accent-b: ${preset.accentB};
    --accent-c: ${preset.accentC};
    --base-top: ${preset.baseTop};
    --base-bottom: ${preset.baseBottom};
  `)

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

  function handleSelectPreset(presetId) {
    if (uiPresets[presetId]) {
      activePreset = presetId
      notify(`Applied ${uiPresets[presetId].name}`)
    }
  }
</script>

<div class="shell" style={themeStyle} onclick={closeMenus} onkeydown={(event) => event.key === 'Escape' && closeMenus()} role="presentation">
  <div class="atmosphere"></div>
  <div class="glow glow-left"></div>
  <div class="glow glow-right"></div>
  <div class="scan-grid"></div>

  <Taskbar
    {time}
    apps={appCatalog}
    {windows}
    presetName={preset.name}
    onToggleStart={toggleStartMenu}
    onToggleCommandPalette={toggleCommandPalette}
    onToggleAI={handleAIAction}
  />

  {#if showStartMenu}
    <StartMenu
      apps={appCatalog}
      windows={windows}
      activePreset={activePreset}
      onSelectPreset={handleSelectPreset}
      onclose={closeMenus}
    />
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
      radial-gradient(1300px 640px at 12% -20%, color-mix(in oklab, var(--accent-a) 20%, transparent), transparent 62%),
      radial-gradient(1100px 520px at 96% -20%, color-mix(in oklab, var(--accent-b) 24%, transparent), transparent 60%),
      radial-gradient(960px 440px at 50% 120%, color-mix(in oklab, var(--accent-c) 13%, transparent), transparent 66%),
      linear-gradient(168deg, var(--base-top) 0%, color-mix(in oklab, var(--base-top) 64%, var(--base-bottom)) 40%, var(--base-bottom) 100%);
  }

  .glow {
    opacity: 0.5;
    animation: drift 26s ease-in-out infinite;
  }

  .glow-left {
    inset: -22% auto auto -16%;
    width: 46vw;
    height: 56vh;
    background: radial-gradient(circle at 28% 36%, color-mix(in oklab, var(--accent-a) 36%, transparent), transparent 70%);
  }

  .glow-right {
    inset: auto -12% -30% auto;
    width: 54vw;
    height: 58vh;
    animation-delay: -8s;
    background: radial-gradient(circle at 60% 30%, color-mix(in oklab, var(--accent-b) 34%, transparent), transparent 72%);
  }

  .scan-grid {
    opacity: 0.14;
    background-image:
      linear-gradient(90deg, color-mix(in oklab, #d2e5ff 6%, transparent) 1px, transparent 1px),
      linear-gradient(0deg, color-mix(in oklab, #d2e5ff 5%, transparent) 1px, transparent 1px);
    background-size: 128px 128px;
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
