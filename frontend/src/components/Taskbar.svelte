<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { time, apps, windows, onToggleStart, onToggleCommandPalette, onToggleAI } = $props()

  const pinnedIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let systemInfo = $state({
    battery: null,
    charging: false,
    username: 'User',
    hostname: 'Windows',
  })

  let pinnedApps = $derived(
    pinnedIds
      .map((id) => apps.find((app) => app.id === id))
      .filter(Boolean)
      .slice(0, 6)
  )

  let liveWindows = $derived(windows.slice(0, 6))

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

  async function loadSystemInfo() {
    try {
      systemInfo = await invoke('get_system_info')
    } catch (error) {
      console.log('System info error:', error)
    }
  }

  async function launchApp(appId) {
    try {
      await invoke('apps_launch', { appId })
    } catch (error) {
      console.log('Launch app error:', error)
    }
  }

  async function focusWindow(windowId) {
    try {
      await invoke('windows_act', { windowId, action: 'focus' })
    } catch (error) {
      console.log('Focus window error:', error)
    }
  }

  async function windowAction(windowId, action, event) {
    event.stopPropagation()
    try {
      await invoke('windows_act', { windowId, action })
    } catch (error) {
      console.log('Window action error:', error)
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

<div class="dock-shell" role="banner">
  <div class="dock-left">
    <button class="logo-btn" onclick={stopAnd(onToggleStart)} title="Open launcher">
      <span class="logo-mark">V</span>
      <span class="logo-text">VORTEX</span>
    </button>

    <button class="action-btn" onclick={stopAnd(onToggleCommandPalette)} title="Search">
      Search
    </button>

    <button class="action-btn" onclick={stopAnd(onToggleAI)} title="AI">
      AI
    </button>
  </div>

  <div class="dock-center">
    <div class="icon-row">
      {#each pinnedApps as app}
        <button class="icon-pill" title={app.name} onclick={stopAnd(() => launchApp(app.id))}>
          <span class="icon-dot"></span>
          <span>{app.name}</span>
        </button>
      {/each}
    </div>

    <div class="window-row">
      {#if liveWindows.length === 0}
        <div class="window-empty">No active windows yet</div>
      {:else}
        {#each liveWindows as window}
          <div
            class="window-pill"
            class:focused={window.isFocused}
            role="button"
            tabindex="0"
            onclick={stopAnd(() => focusWindow(window.id))}
            onkeydown={(event) => event.key === 'Enter' && focusWindow(window.id)}
            title={window.title}
          >
            <span class="window-title">{window.title}</span>
            <div class="window-tools">
              <button type="button" onclick={(event) => windowAction(window.id, 'minimize', event)}>_</button>
              <button type="button" onclick={(event) => windowAction(window.id, 'close', event)}>x</button>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <div class="dock-right">
    <div class="stat-chip">
      <span>{formatDate(time)}</span>
    </div>
    <div class="stat-chip strong">
      <strong>{formatTime(time)}</strong>
    </div>
    <div class="stat-chip">
      <span>{systemInfo.hostname}</span>
      <small>{systemInfo.username}</small>
    </div>
    <div class="stat-chip">
      <span>Battery</span>
      <small>{systemInfo.battery == null ? '--' : `${systemInfo.battery}%`}</small>
    </div>
  </div>
</div>

<style>
  .dock-shell {
    position: absolute;
    left: 14px;
    right: 14px;
    bottom: 12px;
    height: 92px;
    padding: 10px;
    display: grid;
    grid-template-columns: 250px 1fr 260px;
    gap: 10px;
    border-radius: 24px;
    border: 1px solid rgba(164, 190, 255, 0.24);
    background: rgba(12, 18, 28, 0.72);
    backdrop-filter: blur(16px) saturate(130%);
    box-shadow: 0 16px 34px rgba(0, 0, 0, 0.42), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    color: #e8f1ff;
    z-index: 50;
  }

  .dock-left,
  .dock-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .dock-left {
    justify-content: flex-start;
  }

  .dock-right {
    justify-content: flex-end;
  }

  .dock-center {
    display: grid;
    gap: 6px;
    min-width: 0;
  }

  .icon-row,
  .window-row {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
    overflow-x: auto;
  }

  .logo-btn,
  .action-btn,
  .icon-pill,
  .window-pill,
  .stat-chip {
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.05);
    color: inherit;
    border-radius: 14px;
  }

  .logo-btn,
  .action-btn,
  .icon-pill {
    height: 32px;
    padding: 0 10px;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .logo-btn {
    background: linear-gradient(135deg, rgba(102, 167, 255, 0.28), rgba(102, 167, 255, 0.1));
  }

  .logo-mark {
    width: 18px;
    height: 18px;
    display: grid;
    place-items: center;
    border-radius: 7px;
    font-size: 11px;
    font-weight: 700;
    background: rgba(255, 255, 255, 0.16);
  }

  .logo-text {
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.04em;
  }

  .action-btn,
  .icon-pill,
  .window-pill {
    transition: transform 0.15s ease, background 0.15s ease;
  }

  .action-btn:hover,
  .icon-pill:hover,
  .window-pill:hover,
  .logo-btn:hover {
    transform: translateY(-1px);
    background: rgba(255, 255, 255, 0.11);
  }

  .icon-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #7eb8ff;
    box-shadow: 0 0 8px rgba(102, 167, 255, 0.56);
  }

  .icon-pill span:last-child {
    font-size: 11px;
  }

  .window-pill {
    display: flex;
    align-items: center;
    min-width: 180px;
    max-width: 260px;
    height: 32px;
    padding: 0 8px;
    cursor: pointer;
  }

  .window-pill.focused {
    border-color: rgba(102, 167, 255, 0.42);
    background: rgba(102, 167, 255, 0.2);
  }

  .window-title {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 11px;
  }

  .window-tools {
    display: flex;
    gap: 6px;
    margin-left: 8px;
  }

  .window-tools button {
    width: 18px;
    height: 18px;
    border: none;
    border-radius: 6px;
    color: rgba(232, 241, 255, 0.86);
    background: rgba(255, 255, 255, 0.11);
    cursor: pointer;
    font-size: 11px;
    line-height: 1;
  }

  .window-empty {
    height: 32px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    border-radius: 12px;
    color: rgba(215, 229, 255, 0.6);
    border: 1px dashed rgba(255, 255, 255, 0.18);
    font-size: 11px;
  }

  .stat-chip {
    height: 32px;
    padding: 0 10px;
    display: grid;
    align-content: center;
    gap: 1px;
    font-size: 11px;
    color: rgba(219, 232, 255, 0.76);
  }

  .stat-chip strong {
    font-size: 13px;
    color: #f1f7ff;
  }

  .stat-chip small {
    font-size: 10px;
    color: rgba(219, 232, 255, 0.66);
  }

  .stat-chip.strong {
    background: rgba(255, 255, 255, 0.1);
  }
</style>
