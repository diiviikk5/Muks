<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { time, apps, windows, expanded = false, presetName, onToggleStart, onToggleCommandPalette, onToggleAI } = $props()

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

<div class="shell-frame" class:expanded>
  {#if expanded}
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
  {/if}

  <div class="taskbar horizontal bottom" class:compact={!expanded}>
    <div class="weg-items-container">
      {#each pinnedApps as app}
        <button class="weg-item" onclick={stopAnd(() => launchApp(app.id))} title={app.name}>
          <span class="weg-item-icon">{label(app.name)}</span>
          <span class="weg-item-title">{app.name}</span>
          <span class="weg-item-open-sign weg-item-open-sign-active"></span>
        </button>
      {/each}

      <span class="weg-separator weg-separator-1"></span>

      {#if liveWindows.length === 0}
        <div class="weg-empty-state-label">No active windows</div>
      {:else}
        {#each liveWindows as w}
          <div class="weg-item" onclick={stopAnd(() => focusWindow(w.id))} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && focusWindow(w.id)}>
            <span class="weg-item-icon">{label(w.title)}</span>
            <span class="weg-item-title">{w.title}</span>
            <span class="weg-item-open-sign weg-item-open-sign-active" class:weg-item-open-sign-focused={w.isFocused}></span>
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
    width: min(1280px, calc(100vw - 30px));
    margin: 0 auto;
    min-height: 48px;
    border-radius: 14px;
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    align-items: center;
    gap: 8px;
    padding: 6px 9px;
    pointer-events: auto;
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
    min-height: 30px;
    padding: 0 11px;
    border-radius: 9px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 20%, transparent);
    background: color-mix(in oklab, #f0f4ff 7%, transparent);
    color: #eff5ff;
    font-size: 12px;
    display: inline-flex;
    align-items: center;
    letter-spacing: 0.02em;
  }

  .chip,
  .workspace {
    cursor: pointer;
  }

  .chip.strong {
    background:
      linear-gradient(130deg, color-mix(in oklab, var(--accent-a) 22%, transparent), color-mix(in oklab, var(--accent-b) 15%, transparent)),
      color-mix(in oklab, #f0f4ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 42%, transparent);
    font-weight: 600;
  }

  .chip.tiny {
    min-height: 28px;
    padding: 0 9px;
    font-size: 11px;
  }

  .clock {
    min-width: 68px;
    justify-content: center;
    font-weight: 640;
  }

  .workspace-strip {
    display: flex;
    gap: 5px;
    padding: 3px;
    border-radius: 11px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 26%, transparent);
    background: color-mix(in oklab, #edf3ff 5%, transparent);
  }

  .workspace {
    min-height: 24px;
    padding: 0 9px;
    border-color: transparent;
    background: transparent;
    font-size: 11px;
    color: color-mix(in oklab, #eff5ff 74%, transparent);
  }

  .workspace.active {
    background: color-mix(in oklab, var(--accent-a) 24%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 44%, transparent);
  }

  :root {
    --config-item-size: 34px;
    --config-space-between-items: 7px;
    --config-padding: 7px;
    --padding-ratio: 0.15;
    --inner-spacing: calc(var(--config-item-size) * var(--padding-ratio));
    --max-item-size: calc(var(--config-item-size) * 3 + var(--config-space-between-items) * 2);
    --border-radius: calc(var(--config-item-size) * 0.25);
  }

  .taskbar {
    pointer-events: auto;
    position: absolute;
    left: 50%;
    bottom: 0;
    transform: translateX(-50%);
    width: min(1200px, calc(100vw - 30px));
    background: color-mix(in oklab, #eff4ff 8%, transparent);
    border: 1px solid color-mix(in oklab, var(--accent-a) 24%, transparent);
    border-radius: 14px;
    overflow: hidden;
    backdrop-filter: blur(14px) saturate(132%);
  }

  .taskbar > .weg-items-container {
    border-radius: 14px;
    display: flex;
    align-items: center;
    gap: var(--config-space-between-items);
    padding: var(--config-padding);
    overflow-x: auto;
    scrollbar-width: none;
  }

  .shell-frame:not(.expanded) .taskbar {
    width: min(1240px, calc(100vw - 22px));
  }

  .taskbar.compact {
    width: min(760px, calc(100vw - 22px));
  }

  .taskbar.compact .weg-separator {
    display: none;
  }

  .taskbar.compact .weg-item {
    min-width: var(--config-item-size);
    max-width: var(--config-item-size);
    padding: 5px;
    justify-content: center;
  }

  .taskbar.compact .weg-item-title,
  .taskbar.compact .window-actions {
    display: none;
  }

  .taskbar > .weg-items-container::-webkit-scrollbar {
    display: none;
  }

  .weg-separator {
    width: 1px;
    height: 24px;
    background: color-mix(in oklab, var(--accent-a) 28%, transparent);
    flex-shrink: 0;
  }

  .weg-empty-state-label {
    white-space: nowrap;
    line-height: var(--config-item-size);
    font-style: italic;
    color: color-mix(in oklab, #f0f6ff 58%, transparent);
    margin: 0 8px;
    font-size: 12px;
  }

  .weg-item {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: var(--inner-spacing);
    padding: var(--inner-spacing);
    min-width: var(--config-item-size);
    max-width: var(--max-item-size);
    height: var(--config-item-size);
    box-shadow: 0px 2px 3px 0px rgba(0, 0, 0, 0.38);
    border-radius: var(--border-radius);
    border: 1px solid color-mix(in oklab, var(--accent-a) 18%, transparent);
    background: color-mix(in oklab, #eff4ff 8%, transparent);
    transition: background-color 0.2s ease-out;
    color: #eff5ff;
    cursor: pointer;
  }

  .weg-item:hover {
    background: color-mix(in oklab, #eff4ff 12%, transparent);
  }

  .weg-item:active .weg-item-icon {
    transform: scale(0.86);
  }

  .weg-item-icon {
    height: 100%;
    width: unset;
    aspect-ratio: 1/1;
    border-radius: 99px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 700;
    background:
      linear-gradient(135deg, color-mix(in oklab, var(--accent-a) 34%, transparent), color-mix(in oklab, var(--accent-b) 18%, transparent)),
      color-mix(in oklab, #eff4ff 7%, transparent);
    transition: transform 0.2s linear;
  }

  .weg-item-title {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
    font-size: 12px;
    font-weight: 600;
    z-index: 1;
  }

  .weg-item-open-sign {
    position: absolute;
    width: 3px;
    height: 3px;
    border-radius: 6px;
    background-color: color-mix(in oklab, #eff5ff 60%, transparent);
    opacity: 0;
    transition-property: width, height, transform, opacity, background-color, border-radius;
    transition-duration: 0.2s;
    transition-timing-function: linear;
    transform: translateY(-50%);
    top: calc(100% + var(--config-padding) / 2);
  }

  .weg-item-open-sign-active {
    opacity: 1;
  }

  .weg-item-open-sign-focused {
    background-color: color-mix(in oklab, var(--accent-a) 70%, white 10%);
    width: 50%;
  }

  .window-actions {
    display: flex;
    gap: 4px;
    margin-left: 4px;
  }

  .window-actions button {
    width: 18px;
    height: 18px;
    display: grid;
    place-items: center;
    font-size: 10px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 18%, transparent);
    background: color-mix(in oklab, #eff4ff 8%, transparent);
    border-radius: 8px;
    color: #eff5ff;
    cursor: pointer;
    padding: 0;
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
