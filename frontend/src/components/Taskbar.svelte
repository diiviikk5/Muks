<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { time, apps, windows, onToggleStart, onToggleCommandPalette, onToggleAI } = $props()

  const pinnedIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']
  const workspaceLabels = ['Studio', 'Flow', 'Focus', 'Play']

  let systemInfo = $state({ battery: null, username: 'User', hostname: 'Windows' })
  let pinnedApps = $derived(pinnedIds.map((id) => apps.find((app) => app.id === id)).filter(Boolean).slice(0, 6))
  let liveWindows = $derived(windows.slice(0, 7))
  let activeWorkspace = $state(0)

  function formatTime(date) {
    return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: false })
  }

  function formatDate(date) {
    return date.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })
  }

  async function loadSystemInfo() {
    try {
      systemInfo = await invoke('get_system_info')
    } catch {}
  }

  async function launchApp(appId) {
    try {
      await invoke('apps_launch', { appId })
    } catch {}
  }

  async function focusWindow(windowId) {
    try {
      await invoke('windows_act', { windowId, action: 'focus' })
    } catch {}
  }

  async function windowAction(windowId, action, event) {
    event.stopPropagation()
    try {
      await invoke('windows_act', { windowId, action })
    } catch {}
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

<div class="shell-stripe">
  <div class="topbar glass-card">
    <div class="cluster left">
      <button class="seg primary" onclick={stopAnd(onToggleStart)}>Vortex</button>
      <button class="seg" onclick={stopAnd(onToggleCommandPalette)}>Search</button>
    </div>

    <div class="cluster center workspace-strip" role="tablist" aria-label="Workspaces">
      {#each workspaceLabels as label, index}
        <button class="workspace" class:active={index === activeWorkspace} onclick={() => (activeWorkspace = index)}>{label}</button>
      {/each}
    </div>

    <div class="cluster right">
      <button class="seg" onclick={stopAnd(onToggleAI)}>AI</button>
      <div class="meta">
        <strong>{formatTime(time)}</strong>
        <span>{formatDate(time)}</span>
      </div>
      <div class="battery">{systemInfo.battery == null ? '--' : `${systemInfo.battery}%`}</div>
    </div>
  </div>

  <div class="dock glass-card">
    <div class="dock-lane apps">
      {#each pinnedApps as app}
        <button class="dock-chip" onclick={stopAnd(() => launchApp(app.id))} title={app.name}>
          <span class="pulse"></span>
          <span>{app.name}</span>
        </button>
      {/each}
    </div>

    <div class="dock-lane windows">
      {#if liveWindows.length === 0}
        <div class="empty">No active windows</div>
      {:else}
        {#each liveWindows as w}
          <div class="window-chip" class:focused={w.isFocused} onclick={stopAnd(() => focusWindow(w.id))} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && focusWindow(w.id)}>
            <span class="title">{w.title}</span>
            <div class="window-actions">
              <button type="button" onclick={(e) => windowAction(w.id, 'minimize', e)}>_</button>
              <button type="button" onclick={(e) => windowAction(w.id, 'close', e)}>x</button>
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style>
  .shell-stripe {
    position: absolute;
    inset: 12px 14px 12px;
    display: grid;
    grid-template-rows: auto 1fr auto;
    pointer-events: none;
    z-index: 100;
  }

  .glass-card {
    pointer-events: auto;
    border-radius: 22px;
    border: 1px solid color-mix(in oklab, #97b6ff 34%, transparent);
    background:
      linear-gradient(180deg, color-mix(in oklab, #111a33 70%, transparent), color-mix(in oklab, #0b1227 66%, transparent)),
      linear-gradient(140deg, color-mix(in oklab, #7bb7ff 8%, transparent), color-mix(in oklab, #ae7dff 9%, transparent));
    box-shadow: 0 22px 44px rgba(2, 7, 18, 0.55), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(17px) saturate(140%);
  }

  .topbar {
    height: 62px;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    padding: 0 10px;
    gap: 10px;
  }

  .cluster {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .center {
    justify-content: center;
  }

  .right {
    justify-content: flex-end;
  }

  .seg,
  .workspace,
  .battery,
  .meta {
    border-radius: 13px;
    border: 1px solid color-mix(in oklab, #b8ceff 25%, transparent);
    background: color-mix(in oklab, #edf3ff 8%, transparent);
    color: #eef4ff;
    min-height: 36px;
    padding: 0 14px;
    display: inline-flex;
    align-items: center;
    font-size: 13px;
    letter-spacing: 0.02em;
  }

  .seg,
  .workspace {
    cursor: pointer;
    transition: transform 140ms ease, background-color 140ms ease, border-color 140ms ease;
  }

  .seg:hover,
  .workspace:hover,
  .dock-chip:hover,
  .window-chip:hover {
    transform: translateY(-1px);
  }

  .seg.primary {
    background:
      linear-gradient(140deg, color-mix(in oklab, #82bfff 26%, transparent), color-mix(in oklab, #b78fff 20%, transparent)),
      color-mix(in oklab, #edf3ff 8%, transparent);
    border-color: color-mix(in oklab, #8ab9ff 48%, transparent);
    font-weight: 600;
  }

  .workspace-strip {
    gap: 6px;
    padding: 5px;
    border-radius: 15px;
    border: 1px solid color-mix(in oklab, #9ab8ff 32%, transparent);
    background: color-mix(in oklab, #ecf2ff 5%, transparent);
  }

  .workspace {
    border: 1px solid transparent;
    background: transparent;
    min-height: 30px;
    padding: 0 12px;
    font-size: 12px;
    color: color-mix(in oklab, #eff5ff 75%, transparent);
  }

  .workspace.active {
    background: color-mix(in oklab, #9bc4ff 24%, transparent);
    border-color: color-mix(in oklab, #93bcff 44%, transparent);
    color: #f4f8ff;
  }

  .meta {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1px;
    padding: 0 12px;
    min-height: 36px;
  }

  .meta strong {
    font-size: 14px;
    font-weight: 680;
    line-height: 1;
  }

  .meta span {
    color: color-mix(in oklab, #edf4ff 58%, transparent);
    font-size: 11px;
    line-height: 1;
  }

  .battery {
    min-width: 56px;
    justify-content: center;
    font-weight: 600;
  }

  .dock {
    align-self: end;
    min-height: 118px;
    padding: 8px;
    display: grid;
    gap: 8px;
  }

  .dock-lane {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .dock-lane::-webkit-scrollbar {
    display: none;
  }

  .dock-chip,
  .window-chip,
  .empty,
  .window-actions button {
    border: 1px solid color-mix(in oklab, #b8ccff 20%, transparent);
    background: color-mix(in oklab, #eef4ff 7%, transparent);
    border-radius: 12px;
    color: #eff4ff;
  }

  .dock-chip {
    min-height: 40px;
    padding: 0 13px;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    white-space: nowrap;
    font-size: 13px;
    transition: transform 140ms ease, background-color 140ms ease;
  }

  .pulse {
    width: 8px;
    height: 8px;
    border-radius: 99px;
    background: color-mix(in oklab, #8cc4ff 92%, white 8%);
    box-shadow: 0 0 11px color-mix(in oklab, #8cc4ff 72%, transparent);
    flex-shrink: 0;
  }

  .window-chip {
    min-width: 240px;
    max-width: 320px;
    min-height: 40px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    transition: transform 140ms ease, background-color 140ms ease, border-color 140ms ease;
  }

  .window-chip.focused {
    background:
      linear-gradient(130deg, color-mix(in oklab, #8fc4ff 28%, transparent), color-mix(in oklab, #b18bff 18%, transparent)),
      color-mix(in oklab, #eef4ff 7%, transparent);
    border-color: color-mix(in oklab, #94beff 46%, transparent);
  }

  .title {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
    letter-spacing: 0.02em;
  }

  .window-actions {
    display: flex;
    gap: 6px;
  }

  .window-actions button {
    width: 22px;
    height: 22px;
    padding: 0;
    display: grid;
    place-items: center;
    font-size: 12px;
    cursor: pointer;
  }

  .empty {
    min-height: 40px;
    padding: 0 14px;
    display: inline-flex;
    align-items: center;
    color: color-mix(in oklab, #eff4ff 68%, transparent);
    font-size: 12px;
    letter-spacing: 0.02em;
  }

  @media (max-width: 1200px) {
    .workspace-strip {
      display: none;
    }

    .topbar {
      grid-template-columns: 1fr auto;
    }

    .center {
      display: none;
    }

    .window-chip {
      min-width: 210px;
    }
  }
</style>
