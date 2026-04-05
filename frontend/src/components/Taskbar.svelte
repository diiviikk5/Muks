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

  let liveWindows = $derived(windows.slice(0, 5))

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

<div class="top-strip">
  <div class="seg left">
    <button class="seg-btn" onclick={stopAnd(onToggleStart)}>Vortex</button>
    <button class="seg-btn" onclick={stopAnd(onToggleCommandPalette)}>Search</button>
  </div>

  <div class="seg center">
    <span>{formatDate(time)}</span>
    <strong>{formatTime(time)}</strong>
  </div>

  <div class="seg right">
    <button class="seg-btn" onclick={stopAnd(onToggleAI)}>AI</button>
    <span>{systemInfo.battery == null ? '--' : `${systemInfo.battery}%`}</span>
  </div>
</div>

<div class="dock-shell" role="banner">
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
      <div class="window-empty">No active windows</div>
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

<style>
  .top-strip {
    position: absolute;
    left: 14px;
    right: 14px;
    top: 8px;
    height: 32px;
    border-radius: 14px;
    background: rgba(35, 40, 74, 0.72);
    border: 1px solid rgba(175, 196, 255, 0.26);
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    padding: 0 8px;
    gap: 8px;
    color: #dce5ff;
    backdrop-filter: blur(12px) saturate(128%);
    z-index: 60;
  }

  .seg {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
  }

  .seg.center {
    justify-content: center;
    min-width: 170px;
  }

  .seg.right {
    justify-content: flex-end;
  }

  .seg-btn {
    height: 22px;
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.08);
    color: inherit;
    font-size: 11px;
    padding: 0 8px;
    cursor: pointer;
  }

  .seg strong {
    font-size: 12px;
    color: #eff4ff;
  }

  .dock-shell {
    position: absolute;
    left: 14px;
    right: 14px;
    bottom: 12px;
    height: 80px;
    padding: 8px;
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 6px;
    border-radius: 20px;
    border: 1px solid rgba(171, 194, 255, 0.24);
    background: rgba(30, 34, 58, 0.72);
    backdrop-filter: blur(16px) saturate(126%);
    box-shadow: 0 14px 26px rgba(0, 0, 0, 0.38);
    color: #e8efff;
    z-index: 55;
  }

  .icon-row,
  .window-row {
    display: flex;
    align-items: center;
    gap: 6px;
    overflow-x: auto;
    min-width: 0;
  }

  .icon-pill,
  .window-pill,
  .window-empty {
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 11px;
    background: rgba(255, 255, 255, 0.07);
  }

  .icon-pill {
    height: 26px;
    padding: 0 8px;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    color: inherit;
    cursor: pointer;
    font-size: 11px;
  }

  .icon-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #7ec8ff;
    box-shadow: 0 0 8px rgba(126, 200, 255, 0.62);
  }

  .window-pill {
    height: 26px;
    min-width: 170px;
    max-width: 240px;
    padding: 0 7px;
    display: flex;
    align-items: center;
    color: inherit;
    cursor: pointer;
  }

  .window-pill.focused {
    border-color: rgba(126, 200, 255, 0.55);
    background: rgba(126, 200, 255, 0.2);
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
    gap: 5px;
    margin-left: 8px;
  }

  .window-tools button {
    width: 17px;
    height: 17px;
    border: none;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.11);
    color: rgba(234, 242, 255, 0.86);
    font-size: 11px;
    cursor: pointer;
  }

  .window-empty {
    height: 26px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    color: rgba(214, 227, 255, 0.66);
    font-size: 11px;
  }
</style>
