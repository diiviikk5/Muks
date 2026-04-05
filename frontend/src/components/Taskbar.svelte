<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { time, apps, windows, onToggleStart, onToggleCommandPalette, onToggleAI } = $props()

  const pinnedIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let systemInfo = $state({
    battery: null,
    charging: false,
    wifiConnected: null,
    volume: null,
    username: 'User',
    hostname: 'Windows',
  })

  let pinnedApps = $derived(
    pinnedIds
      .map((id) => apps.find((app) => app.id === id))
      .filter(Boolean)
  )

  function formatTime(date) {
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    })
  }

  function formatDate(date) {
    return date.toLocaleDateString('en-US', {
      weekday: 'short',
      day: 'numeric',
      month: 'short',
    })
  }

  async function handleAppClick(app) {
    try {
      await invoke('open_app', { appId: app.id })
    } catch (error) {
      console.log('Open app error:', error)
    }
  }

  async function handleWindowAction(windowId, action) {
    try {
      await invoke('windows_act', { windowId, action })
    } catch (error) {
      console.log('Window action error:', error)
    }
  }

  async function loadSystemInfo() {
    try {
      systemInfo = await invoke('get_system_info')
    } catch (error) {
      console.log('System info error:', error)
    }
  }

  onMount(() => {
    loadSystemInfo()
    const timer = setInterval(loadSystemInfo, 30000)
    return () => clearInterval(timer)
  })

  function stopAnd(fn) {
    return async (event) => {
      event.stopPropagation()
      await fn()
    }
  }
</script>

<div class="taskbar-shell" role="banner">
  <div class="overview-row">
    <button class="brand-card" onclick={stopAnd(onToggleStart)} title="Open VORTEX launcher">
      <div class="brand-mark">V</div>
      <div class="brand-copy">
        <strong>VORTEX Shell</strong>
        <span>{systemInfo.hostname} / {systemInfo.username}</span>
      </div>
    </button>

    <div class="workspace-strip">
      <div class="workspace-card active">
        <span class="workspace-label">Primary</span>
        <strong>{apps.length} indexed apps</strong>
      </div>
      <div class="workspace-card">
        <span class="workspace-label">Launcher</span>
        <strong>Alt+Space search</strong>
      </div>
      <div class="workspace-card">
        <span class="workspace-label">AI</span>
        <strong>Backend pending</strong>
      </div>
      <div class="workspace-card">
        <span class="workspace-label">Live Windows</span>
        <strong>{windows.length} tracked</strong>
      </div>
    </div>

    <div class="quick-panel">
      <button class="quick-chip" onclick={stopAnd(onToggleCommandPalette)} title="Search">
        Search
      </button>
      <button class="quick-chip" onclick={stopAnd(onToggleAI)} title="AI">
        AI
      </button>
      <div class="metric-chip">
        <span>Battery</span>
        <strong>{systemInfo.battery == null ? '--' : `${systemInfo.battery}%`}</strong>
      </div>
      <div class="metric-chip">
        <span>Clock</span>
        <strong>{formatTime(time)}</strong>
      </div>
    </div>
  </div>

  <div class="dock-row">
    <div class="launcher-group">
      <button class="dock-button primary" onclick={stopAnd(onToggleStart)} title="Launcher">
        Start
      </button>
      <button class="dock-button" onclick={stopAnd(onToggleCommandPalette)} title="Search">
        Search
      </button>
    </div>

    <div class="apps-dock">
      {#each pinnedApps as app}
        <button class="app-pill" title={app.name} onclick={stopAnd(() => handleAppClick(app))}>
          <span class="app-glyph"></span>
          <span>{app.name}</span>
        </button>
      {/each}
    </div>

    <div class="system-cluster">
      <div class="system-chip">
        <span>{formatDate(time)}</span>
      </div>
      <div class="system-chip emphasis">
        <strong>{formatTime(time)}</strong>
      </div>
    </div>
  </div>

  {#if windows.length > 0}
    <div class="window-row">
      {#each windows.slice(0, 8) as window}
        <div
          class="window-pill"
          class:focused={window.isFocused}
          title={window.title}
          onclick={stopAnd(() => handleWindowAction(window.id, 'focus'))}
          onkeydown={(event) => event.key === 'Enter' && handleWindowAction(window.id, 'focus')}
          role="button"
          tabindex="0"
        >
          <span class="window-dot"></span>
          <span>{window.title}</span>
          <span class="window-actions">
            <button type="button" onclick={stopAnd(() => handleWindowAction(window.id, 'minimize'))}>_</button>
            <button type="button" onclick={stopAnd(() => handleWindowAction(window.id, 'close'))}>x</button>
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .taskbar-shell {
    position: absolute;
    inset: 12px 14px 12px 14px;
    display: grid;
    gap: 10px;
    padding: 12px 14px;
    border-radius: 26px;
    border: 1px solid rgba(126, 170, 255, 0.18);
    background:
      linear-gradient(180deg, rgba(24, 30, 42, 0.98), rgba(11, 15, 21, 0.98)),
      radial-gradient(circle at top right, rgba(100, 165, 255, 0.15), transparent 35%);
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.06),
      0 18px 34px rgba(0, 0, 0, 0.35);
    color: #ebf2ff;
  }

  .overview-row,
  .dock-row {
    display: grid;
    gap: 10px;
    align-items: center;
  }

  .overview-row {
    grid-template-columns: 240px 1fr 320px;
  }

  .dock-row {
    grid-template-columns: 180px 1fr 170px;
  }

  .brand-card,
  .quick-chip,
  .dock-button,
  .app-pill {
    border: none;
    cursor: pointer;
  }

  .brand-card {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
    padding: 12px;
    border-radius: 18px;
    color: inherit;
    background: linear-gradient(135deg, rgba(91, 155, 255, 0.22), rgba(91, 155, 255, 0.08));
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
  }

  .brand-mark {
    width: 38px;
    height: 38px;
    display: grid;
    place-items: center;
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.1);
    font-weight: 800;
    letter-spacing: 0.08em;
  }

  .brand-copy {
    display: grid;
    min-width: 0;
    text-align: left;
  }

  .brand-copy strong,
  .workspace-card strong,
  .metric-chip strong {
    font-size: 13px;
    font-weight: 700;
  }

  .brand-copy span,
  .workspace-label,
  .metric-chip span {
    color: rgba(220, 232, 255, 0.64);
    font-size: 11px;
  }

  .workspace-strip {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
  }

  .workspace-card,
  .metric-chip,
  .system-chip {
    display: grid;
    gap: 2px;
    padding: 10px 12px;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.04);
  }

  .workspace-card.active {
    background: rgba(91, 155, 255, 0.12);
    border-color: rgba(91, 155, 255, 0.2);
  }

  .quick-panel,
  .launcher-group,
  .apps-dock,
  .system-cluster {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .quick-panel {
    justify-content: flex-end;
  }

  .quick-chip,
  .dock-button {
    padding: 10px 14px;
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.06);
    color: #e8f0ff;
    font-size: 12px;
    transition: background 0.16s ease, transform 0.16s ease;
  }

  .quick-chip:hover,
  .dock-button:hover,
  .app-pill:hover,
  .brand-card:hover,
  .window-pill:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: translateY(-1px);
  }

  .dock-button.primary {
    background: linear-gradient(135deg, rgba(91, 155, 255, 0.3), rgba(91, 155, 255, 0.12));
  }

  .apps-dock {
    min-width: 0;
    overflow: hidden;
  }

  .app-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    padding: 11px 13px;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.05);
    color: #edf3ff;
    font-size: 12px;
    white-space: nowrap;
  }

  .app-glyph {
    width: 10px;
    height: 10px;
    border-radius: 999px;
    background: linear-gradient(135deg, #8bc0ff, #4d8dff);
    box-shadow: 0 0 10px rgba(91, 155, 255, 0.5);
    flex: 0 0 auto;
  }

  .system-cluster {
    justify-content: flex-end;
  }

  .system-chip {
    min-width: 0;
    color: rgba(235, 242, 255, 0.8);
    font-size: 11px;
  }

  .system-chip.emphasis {
    background: rgba(255, 255, 255, 0.08);
  }

  .window-row {
    display: flex;
    gap: 8px;
    overflow-x: auto;
  }

  .window-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    border: none;
    border-radius: 13px;
    padding: 8px 10px;
    min-width: 180px;
    max-width: 260px;
    background: rgba(255, 255, 255, 0.05);
    color: #dfe9ff;
    cursor: pointer;
    font-size: 11px;
  }

  .window-pill.focused {
    background: rgba(91, 155, 255, 0.2);
    border: 1px solid rgba(91, 155, 255, 0.38);
  }

  .window-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #8bc0ff;
    box-shadow: 0 0 8px rgba(91, 155, 255, 0.6);
    flex: 0 0 auto;
  }

  .window-pill > span:nth-child(2) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    text-align: left;
    flex: 1;
  }

  .window-actions {
    display: flex;
    gap: 8px;
  }

  .window-actions button {
    border: none;
    border-radius: 8px;
    width: 20px;
    height: 20px;
    color: rgba(230, 239, 255, 0.82);
    background: rgba(255, 255, 255, 0.08);
    font-weight: 700;
    cursor: pointer;
  }
</style>
