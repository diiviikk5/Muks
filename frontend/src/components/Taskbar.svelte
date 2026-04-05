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
    pinnedIds.map((id) => apps.find((app) => app.id === id)).filter(Boolean).slice(0, 7)
  )

  let liveWindows = $derived(windows.slice(0, 6))

  function formatTime(date) {
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
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

<div class="topbar">
  <div class="left-tools">
    <button class="tool primary" onclick={stopAnd(onToggleStart)}>Vortex</button>
    <button class="tool" onclick={stopAnd(onToggleCommandPalette)}>Search</button>
  </div>

  <div class="clock-pill">{formatTime(time)}</div>

  <div class="right-tools">
    <button class="tool" onclick={stopAnd(onToggleAI)}>AI</button>
    <div class="stat">{systemInfo.battery == null ? '--' : `${systemInfo.battery}%`}</div>
  </div>
</div>

<div class="dock-shell" role="banner">
  <div class="track pinned-track">
    {#each pinnedApps as app}
      <button class="app-pill" title={app.name} onclick={stopAnd(() => launchApp(app.id))}>
        <span class="dot"></span>
        <span>{app.name}</span>
      </button>
    {/each}
  </div>

  <div class="track windows-track">
    {#if liveWindows.length === 0}
      <div class="empty">No active windows</div>
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
          <span class="title">{window.title}</span>
          <div class="actions">
            <button type="button" onclick={(event) => windowAction(window.id, 'minimize', event)}>_</button>
            <button type="button" onclick={(event) => windowAction(window.id, 'close', event)}>x</button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .topbar {
    position: absolute;
    top: 8px;
    left: 12px;
    right: 12px;
    height: 34px;
    border-radius: 15px;
    border: 1px solid rgba(173, 194, 255, 0.24);
    background: rgba(33, 37, 64, 0.72);
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    padding: 0 8px;
    gap: 8px;
    backdrop-filter: blur(12px) saturate(125%);
    z-index: 80;
  }

  .left-tools,
  .right-tools {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .right-tools {
    justify-content: flex-end;
  }

  .tool,
  .stat,
  .clock-pill {
    height: 24px;
    border-radius: 9px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(255, 255, 255, 0.08);
    color: #e3eaff;
    font-size: 11px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0 9px;
  }

  .tool {
    cursor: pointer;
  }

  .tool.primary {
    background: linear-gradient(135deg, rgba(127, 200, 255, 0.25), rgba(230, 168, 211, 0.16));
  }

  .clock-pill {
    min-width: 120px;
    font-weight: 700;
    letter-spacing: 0.04em;
  }

  .dock-shell {
    position: absolute;
    left: 12px;
    right: 12px;
    bottom: 10px;
    height: 84px;
    border-radius: 22px;
    border: 1px solid rgba(173, 194, 255, 0.24);
    background: rgba(31, 35, 60, 0.74);
    backdrop-filter: blur(16px) saturate(130%);
    box-shadow: 0 14px 28px rgba(0, 0, 0, 0.42);
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 5px;
    padding: 8px;
    color: #e8efff;
    z-index: 70;
  }

  .track {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
    overflow-x: auto;
  }

  .app-pill,
  .window-pill,
  .empty {
    height: 28px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.11);
    background: rgba(255, 255, 255, 0.08);
  }

  .app-pill {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
    color: inherit;
    cursor: pointer;
    font-size: 11px;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #7fc8ff;
    box-shadow: 0 0 8px rgba(127, 200, 255, 0.6);
  }

  .window-pill {
    min-width: 170px;
    max-width: 240px;
    padding: 0 7px;
    display: flex;
    align-items: center;
    color: inherit;
    cursor: pointer;
  }

  .window-pill.focused {
    border-color: rgba(127, 200, 255, 0.55);
    background: rgba(127, 200, 255, 0.22);
  }

  .title {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 11px;
  }

  .actions {
    display: flex;
    gap: 5px;
    margin-left: 8px;
  }

  .actions button {
    width: 17px;
    height: 17px;
    border: none;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.12);
    color: rgba(232, 240, 255, 0.88);
    font-size: 11px;
    cursor: pointer;
  }

  .empty {
    padding: 0 10px;
    display: inline-flex;
    align-items: center;
    color: rgba(202, 214, 246, 0.7);
    font-size: 11px;
  }
</style>
