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
  let statusMessage = $state('')
  let themes = $state([])
  let windows = $state([])

  let shellState = $state({
    mode: 'normal',
    healthy: false,
    uptimeSeconds: 0,
    startupTimeMs: 0,
    activeWorkspaceId: 'ws-1',
    performance: {
      appIndexSize: 0,
      launchCount: 0,
      commandCount: 0,
      indexRefreshCount: 0,
      lastIndexedEpochMs: null,
      lastRefreshDeltaApps: 0,
    },
  })

  async function updateMenuState() {
    try {
      await invoke('set_menu_open', { isOpen: showStartMenu || showCommandPalette })
    } catch (error) {
      console.log('Failed to resize window:', error)
    }
  }

  async function loadShellState() {
    try {
      shellState = await invoke('shell_get_state')
    } catch (error) {
      console.log('Failed to load shell state:', error)
    }
  }

  async function loadAppCatalog() {
    try {
      appCatalog = await invoke('apps_search', { query: '', limit: 300 })
    } catch (error) {
      console.log('Failed to load app catalog:', error)
    }
  }

  async function loadThemes() {
    try {
      themes = await invoke('themes_list')
    } catch (error) {
      console.log('Failed to load themes:', error)
    }
  }

  async function loadWindows() {
    try {
      windows = await invoke('windows_sync')
    } catch (error) {
      console.log('Failed to load windows:', error)
    }
  }

  async function setMode(mode) {
    try {
      shellState = await invoke('shell_set_mode', { mode })
      showStatus(`Shell mode: ${mode}`)
    } catch (error) {
      console.log('Failed to set shell mode:', error)
    }
  }

  async function applyTheme(themeId) {
    try {
      await invoke('themes_apply', { themeId })
      await loadThemes()
      showStatus(`Theme applied: ${themeId}`)
    } catch (error) {
      console.log('Failed to apply theme:', error)
    }
  }

  async function refreshIndex() {
    try {
      const stats = await invoke('apps_refresh_index')
      await loadAppCatalog()
      await loadShellState()
      showStatus(`Index refreshed: ${stats.newCount} apps (${stats.delta >= 0 ? '+' : ''}${stats.delta})`)
    } catch (error) {
      console.log('Failed to refresh app index:', error)
    }
  }

  function showStatus(message) {
    statusMessage = message
    setTimeout(() => {
      if (statusMessage === message) {
        statusMessage = ''
      }
    }, 2800)
  }

  function formatSince(epochMs) {
    if (!epochMs) return 'Never'
    const diff = Date.now() - epochMs
    const sec = Math.max(0, Math.round(diff / 1000))
    if (sec < 60) return `${sec}s ago`
    const min = Math.round(sec / 60)
    return `${min}m ago`
  }

  onMount(() => {
    updateMenuState()
    loadShellState()
    loadAppCatalog()
    loadThemes()
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
      listen('system.state.changed', (event) => {
        shellState = event.payload
      }),
      listen('apps.index.updated', async () => {
        await loadAppCatalog()
        await loadShellState()
      }),
      listen('workspace.changed', () => {
        loadShellState()
      }),
      listen('windows.changed', (event) => {
        windows = event.payload
      }),
      listen('theme.changed', async () => {
        await loadThemes()
      }),
      listen('plugin.health', (event) => {
        const name = event.payload?.name ?? 'Plugin'
        showStatus(`${name} status updated`)
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
    showStatus('AI stays disabled until there is a real local or cloud backend wired into the shell.')
  }

  function handleAIRequest(query) {
    showStatus(`Saved as an AI placeholder request: ${query}`)
  }
</script>

<div class="shell-container" onclick={closeMenus} onkeydown={(event) => event.key === 'Escape' && closeMenus()} role="presentation">
  <div class="shell-chrome"></div>

  <div class="runtime-ribbon">
    <div class="runtime-chip" class:ok={shellState.healthy}>
      <span>Shell</span>
      <strong>{shellState.healthy ? 'Healthy' : 'Starting'}</strong>
    </div>
    <div class="runtime-chip">
      <span>Mode</span>
      <strong>{shellState.mode}</strong>
    </div>
    <div class="runtime-chip">
      <span>Indexed</span>
      <strong>{shellState.performance.appIndexSize}</strong>
    </div>
    <div class="runtime-chip">
      <span>Startup</span>
      <strong>{shellState.startupTimeMs}ms</strong>
    </div>
    <div class="runtime-chip">
      <span>Refresh</span>
      <strong>{formatSince(shellState.performance.lastIndexedEpochMs)}</strong>
    </div>
    <div class="runtime-chip">
      <span>Delta</span>
      <strong>{shellState.performance.lastRefreshDeltaApps >= 0 ? '+' : ''}{shellState.performance.lastRefreshDeltaApps}</strong>
    </div>
  </div>

  <div
    class="control-dock"
    onclick={(event) => event.stopPropagation()}
    onkeydown={(event) => event.key === 'Escape' && closeMenus()}
    role="presentation"
  >
    <div class="mode-group">
      <button class:active={shellState.mode === 'normal'} onclick={() => setMode('normal')}>Normal</button>
      <button class:active={shellState.mode === 'performance'} onclick={() => setMode('performance')}>Performance</button>
      <button class:active={shellState.mode === 'safe'} onclick={() => setMode('safe')}>Safe</button>
    </div>

    <div class="theme-group">
      {#each themes as theme}
        <button class:active={theme.isActive} onclick={() => applyTheme(theme.id)}>{theme.name}</button>
      {/each}
    </div>

    <button class="refresh-button" onclick={refreshIndex}>Refresh Index</button>
  </div>

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
      <div class="status-pulse"></div>
      <span>{statusMessage}</span>
    </div>
  {/if}
</div>

<style>
  .shell-container {
    position: relative;
    height: 100vh;
    width: 100%;
    overflow: hidden;
    background:
      radial-gradient(circle at 15% -30%, rgba(83, 162, 255, 0.18), transparent 36%),
      radial-gradient(circle at 88% -50%, rgba(64, 200, 255, 0.14), transparent 40%),
      linear-gradient(180deg, rgba(12, 17, 26, 0.98), rgba(8, 11, 16, 1));
  }

  .shell-chrome {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(100deg, rgba(255, 255, 255, 0.05) 0, rgba(255, 255, 255, 0) 18%, rgba(255, 255, 255, 0.03) 100%),
      radial-gradient(circle at 82% -60%, rgba(86, 173, 255, 0.18), transparent 52%);
    pointer-events: none;
  }

  .runtime-ribbon {
    position: absolute;
    top: 10px;
    left: 16px;
    right: 16px;
    z-index: 40;
    display: flex;
    gap: 8px;
    overflow-x: auto;
  }

  .runtime-chip {
    display: grid;
    gap: 1px;
    min-width: 118px;
    padding: 8px 10px;
    border-radius: 13px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(9, 12, 18, 0.7);
    backdrop-filter: blur(12px) saturate(135%);
    color: #c9d7f1;
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .runtime-chip strong {
    font-size: 12px;
    text-transform: none;
    color: #f1f7ff;
    letter-spacing: normal;
  }

  .runtime-chip.ok {
    border-color: rgba(99, 235, 191, 0.3);
    box-shadow: inset 0 0 0 1px rgba(99, 235, 191, 0.12);
  }

  .control-dock {
    position: absolute;
    top: 76px;
    left: 16px;
    right: 16px;
    z-index: 41;
    display: flex;
    gap: 10px;
    align-items: center;
    flex-wrap: wrap;
  }

  .mode-group,
  .theme-group {
    display: flex;
    gap: 8px;
    padding: 8px;
    border-radius: 14px;
    background: rgba(9, 12, 18, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(12px) saturate(135%);
  }

  .mode-group button,
  .theme-group button,
  .refresh-button {
    border: none;
    padding: 8px 12px;
    border-radius: 10px;
    font-size: 12px;
    color: #e6efff;
    background: rgba(255, 255, 255, 0.06);
    cursor: pointer;
    transition: transform 0.14s ease, background 0.14s ease;
  }

  .mode-group button:hover,
  .theme-group button:hover,
  .refresh-button:hover,
  .mode-group button.active,
  .theme-group button.active {
    transform: translateY(-1px);
    background: rgba(91, 155, 255, 0.28);
  }

  .refresh-button {
    background: linear-gradient(135deg, rgba(91, 155, 255, 0.32), rgba(74, 128, 255, 0.14));
  }

  .status-indicator {
    position: fixed;
    right: 20px;
    bottom: 124px;
    display: flex;
    align-items: center;
    gap: 8px;
    max-width: 360px;
    padding: 10px 14px;
    background: rgba(6, 8, 12, 0.92);
    border: 1px solid rgba(91, 155, 255, 0.25);
    border-radius: 16px;
    color: #d8e6ff;
    font-size: 12px;
    z-index: 1000;
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.35);
  }

  .status-pulse {
    width: 8px;
    height: 8px;
    border-radius: 999px;
    background: #5b9bff;
    box-shadow: 0 0 0 0 rgba(91, 155, 255, 0.45);
    animation: pulse 1.8s infinite;
    flex: 0 0 auto;
  }

  @keyframes pulse {
    0%, 100% { box-shadow: 0 0 0 0 rgba(91, 155, 255, 0.45); }
    50% { box-shadow: 0 0 0 8px rgba(91, 155, 255, 0); }
  }
</style>
