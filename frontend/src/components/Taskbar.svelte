<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { time, apps, windows, presetName, onToggleStart, onToggleCommandPalette, onToggleAI } = $props()

  const pinnedIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']
  const workspaceLabels = ['Studio', 'Flow', 'Focus', 'Play']

  let systemInfo = $state({ battery: null, username: 'User', hostname: 'Windows' })
  let pinnedApps = $derived(pinnedIds.map((id) => apps.find((app) => app.id === id)).filter(Boolean).slice(0, 6))
  let liveWindows = $derived(windows.slice(0, 5))
  let activeWorkspace = $state(0)

  function formatTime(date) {
    return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: false })
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

  function label(text) {
    const t = (text || '').trim()
    if (!t) return 'A'
    return t.slice(0, 1).toUpperCase()
  }
</script>

<div class="shell-frame">
  <div class="topbar glass">
    <div class="left-cluster">
      <button class="chip strong" onclick={stopAnd(onToggleStart)}>Vortex</button>
      <button class="chip" onclick={stopAnd(onToggleCommandPalette)}>Search</button>
    </div>

    <div class="workspace-strip" role="tablist" aria-label="Workspaces">
      {#each workspaceLabels as labelText, index}
        <button class="workspace" class:active={index === activeWorkspace} onclick={() => (activeWorkspace = index)}>{labelText}</button>
      {/each}
    </div>

    <div class="right-cluster">
      <button class="chip" onclick={stopAnd(onToggleAI)}>AI</button>
      <div class="clock">{formatTime(time)}</div>
      <div class="chip tiny">{presetName}</div>
      <div class="chip tiny">{systemInfo.battery == null ? '--' : `${systemInfo.battery}%`}</div>
    </div>
  </div>

  <div class="dock glass">
    <div class="dock-lane">
      {#each pinnedApps as app}
        <button class="app-chip" onclick={stopAnd(() => launchApp(app.id))} title={app.name}>
          <span class="app-icon">{label(app.name)}</span>
          <span>{app.name}</span>
        </button>
      {/each}

      <span class="divider" aria-hidden="true"></span>

      {#if liveWindows.length === 0}
        <div class="empty">No active windows</div>
      {:else}
        {#each liveWindows as w}
          <div class="window-chip" class:focused={w.isFocused} onclick={stopAnd(() => focusWindow(w.id))} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && focusWindow(w.id)}>
            <span class="window-icon">{label(w.title)}</span>
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
  .shell-frame {
    position: absolute;
    inset: 10px 16px 10px;
    pointer-events: none;
    z-index: 110;
  }

  .glass {
    pointer-events: auto;
    border: 1px solid color-mix(in oklab, var(--accent-a) 30%, transparent);
    background:
      linear-gradient(180deg, color-mix(in oklab, #10152b 78%, transparent), color-mix(in oklab, #0b1022 72%, transparent)),
      linear-gradient(140deg, color-mix(in oklab, var(--accent-a) 6%, transparent), color-mix(in oklab, var(--accent-b) 8%, transparent));
    box-shadow: 0 16px 30px rgba(1, 4, 12, 0.48), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(14px) saturate(132%);
  }

  .topbar {
    width: min(1320px, calc(100vw - 38px));
    margin: 0 auto;
    min-height: 52px;
    border-radius: 16px;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 10px;
    padding: 7px 10px;
  }

  .left-cluster,
  .right-cluster {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .right-cluster {
    justify-content: flex-end;
  }

  .chip,
  .clock,
  .workspace {
    min-height: 32px;
    padding: 0 12px;
    border-radius: 10px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 20%, transparent);
    background: color-mix(in oklab, #f0f4ff 7%, transparent);
    color: #eff5ff;
    font-size: 13px;
    display: inline-flex;
    align-items: center;
    letter-spacing: 0.02em;
  }

  .chip,
  .workspace,
  .app-chip,
  .window-chip {
    transition: transform 120ms ease, border-color 120ms ease, background-color 120ms ease;
  }

  .chip {
    cursor: pointer;
  }

  .chip:hover,
  .workspace:hover,
  .app-chip:hover,
  .window-chip:hover {
    transform: translateY(-1px);
  }

  .chip.strong {
    background:
      linear-gradient(130deg, color-mix(in oklab, var(--accent-a) 22%, transparent), color-mix(in oklab, var(--accent-b) 15%, transparent)),
      color-mix(in oklab, #f0f4ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 42%, transparent);
    font-weight: 600;
  }

  .chip.tiny {
    min-height: 30px;
    padding: 0 10px;
    font-size: 12px;
  }

  .clock {
    min-width: 72px;
    justify-content: center;
    font-weight: 640;
  }

  .workspace-strip {
    display: flex;
    gap: 5px;
    padding: 3px;
    border-radius: 12px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 26%, transparent);
    background: color-mix(in oklab, #edf3ff 5%, transparent);
  }

  .workspace {
    min-height: 26px;
    padding: 0 10px;
    border-color: transparent;
    background: transparent;
    font-size: 12px;
    cursor: pointer;
    color: color-mix(in oklab, #eff5ff 74%, transparent);
  }

  .workspace.active {
    background: color-mix(in oklab, var(--accent-a) 24%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 44%, transparent);
  }

  .dock {
    position: absolute;
    left: 50%;
    bottom: 0;
    transform: translateX(-50%);
    width: min(1240px, calc(100vw - 34px));
    border-radius: 18px;
    min-height: 56px;
    padding: 7px;
  }

  .dock-lane {
    display: flex;
    align-items: center;
    gap: 7px;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .dock-lane::-webkit-scrollbar {
    display: none;
  }

  .divider {
    width: 1px;
    height: 26px;
    background: color-mix(in oklab, var(--accent-a) 30%, transparent);
    flex-shrink: 0;
    margin: 0 3px;
  }

  .app-chip,
  .window-chip,
  .empty,
  .window-actions button,
  .app-icon,
  .window-icon {
    border: 1px solid color-mix(in oklab, var(--accent-a) 18%, transparent);
    background: color-mix(in oklab, #eff4ff 7%, transparent);
    color: #eff5ff;
    border-radius: 10px;
  }

  .app-chip {
    min-height: 34px;
    padding: 0 8px;
    display: inline-flex;
    align-items: center;
    gap: 7px;
    font-size: 12px;
    white-space: nowrap;
    cursor: pointer;
  }

  .app-icon,
  .window-icon {
    width: 18px;
    height: 18px;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 600;
    padding: 0;
    flex-shrink: 0;
    background:
      linear-gradient(135deg, color-mix(in oklab, var(--accent-a) 34%, transparent), color-mix(in oklab, var(--accent-b) 18%, transparent)),
      color-mix(in oklab, #eff4ff 7%, transparent);
  }

  .window-chip {
    min-height: 34px;
    min-width: 220px;
    max-width: 310px;
    padding: 0 8px;
    display: flex;
    align-items: center;
    gap: 7px;
    cursor: pointer;
  }

  .window-chip.focused {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 24%, transparent), color-mix(in oklab, var(--accent-b) 16%, transparent)),
      color-mix(in oklab, #eff4ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 48%, transparent);
  }

  .title {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
  }

  .window-actions {
    display: flex;
    gap: 4px;
  }

  .window-actions button {
    width: 18px;
    height: 18px;
    display: grid;
    place-items: center;
    font-size: 10px;
    cursor: pointer;
    padding: 0;
  }

  .empty {
    min-height: 34px;
    padding: 0 10px;
    display: inline-flex;
    align-items: center;
    font-size: 12px;
    color: color-mix(in oklab, #eff5ff 70%, transparent);
  }

  @media (max-width: 1160px) {
    .workspace-strip {
      display: none;
    }

    .topbar {
      grid-template-columns: 1fr auto;
    }
  }
</style>
