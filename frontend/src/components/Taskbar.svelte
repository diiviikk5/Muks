<script>
  import { invoke } from '@tauri-apps/api/core'

  let { time, apps, windows, expanded = false, onToggleStart, onToggleCommandPalette, onToggleAI, presetName } = $props()

  const pinnedIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let pinnedApps = $derived(pinnedIds.map((id) => apps.find((item) => item.id === id)).filter(Boolean))
  let liveWindows = $derived(windows.slice(0, 12))

  function initial(value) {
    const text = (value || '').trim()
    return text ? text[0].toUpperCase() : '?'
  }

  function stopAnd(handler) {
    return (event) => {
      event.stopPropagation()
      handler()
    }
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
</script>

<div class="toolbar-host">
  <div class="ft-bar top" data-has-margin="true">
    <div class="ft-bar-left">
      <div class="ft-bar-item ft-bar-item-clickable" role="button" tabindex="0" onclick={stopAnd(onToggleStart)} onkeydown={(e) => e.key === 'Enter' && onToggleStart()}>
        <div class="ft-bar-item-content"><span>Vortex</span></div>
      </div>
      <div class="ft-bar-item ft-bar-item-clickable" role="button" tabindex="0" onclick={stopAnd(onToggleCommandPalette)} onkeydown={(e) => e.key === 'Enter' && onToggleCommandPalette()}>
        <div class="ft-bar-item-content"><span>Search</span></div>
      </div>
    </div>
    <div class="ft-bar-center">
      <div class="ft-bar-item"><div class="ft-bar-item-content"><span>{time.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: false })}</span></div></div>
    </div>
    <div class="ft-bar-right">
      <div class="ft-bar-item"><div class="ft-bar-item-content"><span>{presetName}</span></div></div>
      <div class="ft-bar-item ft-bar-item-clickable" role="button" tabindex="0" onclick={stopAnd(onToggleAI)} onkeydown={(e) => e.key === 'Enter' && onToggleAI()}>
        <div class="ft-bar-item-content"><span>AI</span></div>
      </div>
    </div>
  </div>
</div>

<div class="dock-host" class:expanded>
  <div class="taskbar horizontal bottom full-width">
    <div class="weg-items-container">
      <div class="weg-items">
        {#each pinnedApps as app}
          <div class="weg-item-drag-container">
            <button class="weg-item" onclick={stopAnd(() => launchApp(app.id))} title={app.name}>
              <span class="weg-item-icon weg-item-start-icon">{initial(app.name)}</span>
              <span class="weg-item-title">{app.name}</span>
              <span class="weg-item-open-sign"></span>
            </button>
          </div>
        {/each}

        <div class="weg-item-drag-container"><span class="weg-separator weg-separator-1 visible"></span></div>

        {#if liveWindows.length === 0}
          <div class="weg-empty-state-label">No open windows</div>
        {:else}
          {#each liveWindows as win}
            <div class="weg-item-drag-container">
              <button class="weg-item" onclick={stopAnd(() => focusWindow(win.id))} title={win.title}>
                <span class="weg-item-icon">{initial(win.title)}</span>
                <span class="weg-item-title">{win.title}</span>
                <span
                  class="weg-item-open-sign weg-item-open-sign-active"
                  class:weg-item-open-sign-focused={win.isFocused}
                ></span>
              </button>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .toolbar-host {
    pointer-events: none;
    position: fixed;
    inset: 0;
    z-index: 120;
  }

  .ft-bar {
    pointer-events: auto;
  }

  .dock-host {
    pointer-events: none;
    position: fixed;
    inset: 0;
    z-index: 130;
  }

  .taskbar {
    pointer-events: auto;
  }

  .weg-item {
    border: 0;
    cursor: pointer;
  }

  .weg-item-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 0.72rem;
    font-weight: 700;
  }

  .dock-host:not(.expanded) .weg-item-title,
  .dock-host:not(.expanded) .weg-separator,
  .dock-host:not(.expanded) .weg-empty-state-label,
  .dock-host:not(.expanded) .weg-item-drag-container:nth-child(n + 8) {
    display: none;
  }

  .dock-host:not(.expanded) .taskbar.full-width {
    width: min-content;
  }
</style>
