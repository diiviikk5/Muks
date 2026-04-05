<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, windows = [], activePreset, onSelectPreset, onclose } = $props()
  let query = $state('')
  let viewMode = $state('favorites')

  const featuredIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let featuredApps = $derived(apps.filter((app) => featuredIds.includes(app.id)).slice(0, 8))
  let filteredApps = $derived(
    query.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 80)
      : apps.slice(0, 80)
  )
  let liveWindows = $derived(windows.slice(0, 5))

  async function launch(app) {
    try {
      await invoke('apps_launch', { appId: app.id })
    } catch (error) {
      console.log('Launch error:', error)
    }
    onclose()
  }

  function initial(name) {
    return (name || '?').slice(0, 1).toUpperCase()
  }
</script>

<div class="apps-menu" role="dialog" tabindex="-1" aria-label="Launcher" onclick={(event) => event.stopPropagation()} onkeydown={(event) => event.key === 'Escape' && onclose()}>
  <div class="apps-menu-header">
    <input type="search" placeholder="Applications" bind:value={query} />

    <div class="header-actions">
      <button class:active={viewMode === 'favorites'} onclick={() => (viewMode = 'favorites')}>Favorites</button>
      <button class:active={viewMode === 'all'} onclick={() => (viewMode = 'all')}>All Apps</button>
    </div>

    <button onclick={onclose}>Close</button>
  </div>

  <div class="apps-menu-body">
    <div class="body-main">
      {#if viewMode === 'favorites'}
        <h3>Favorites</h3>
        <div class="pinned-view-list">
          {#each featuredApps as app}
            <button class="app" onclick={() => launch(app)}>
              <span class="app-icon">{initial(app.name)}</span>
              <span class="app-name">{app.name}</span>
            </button>
          {/each}
        </div>
      {/if}

      <h3>{viewMode === 'favorites' ? 'All Apps' : 'Catalog'}</h3>
      <div class="all-apps-view-list">
        {#each filteredApps as app}
          <button class="app-row" onclick={() => launch(app)}>
            <span class="row-main">
              <span class="app-icon small">{initial(app.name)}</span>
              <span>{app.name}</span>
            </span>
            <small>{app.source}</small>
          </button>
        {/each}
      </div>
    </div>

    <aside class="body-side">
      <section>
        <h3>Profiles</h3>
        <div class="stack">
          <button class:active={activePreset === 'graphite-flare'} onclick={() => onSelectPreset('graphite-flare')}>Graphite Flare</button>
          <button class:active={activePreset === 'forest-glass'} onclick={() => onSelectPreset('forest-glass')}>Forest Glass</button>
          <button class:active={activePreset === 'rose-dusk'} onclick={() => onSelectPreset('rose-dusk')}>Rose Dusk</button>
        </div>
      </section>

      <section>
        <h3>Live Windows</h3>
        <div class="stack">
          {#if liveWindows.length === 0}
            <div class="empty-box">No active windows</div>
          {:else}
            {#each liveWindows as item}
              <div class="window-row">{item.title}</div>
            {/each}
          {/if}
        </div>
      </section>
    </aside>
  </div>

  <div class="apps-menu-footer">
    <div class="apps-menu-footer-left">
      <button class="user-profile">diviikk5</button>
      <button class="user-profile">Workspace A</button>
    </div>

    <div class="apps-menu-footer-right">
      <button>Settings</button>
      <button>Power</button>
      <button>Expand</button>
    </div>
  </div>
</div>

<style>
  :root {
    --columns: 4;
  }

  .apps-menu {
    position: absolute;
    left: 50%;
    top: 70px;
    bottom: 66px;
    transform: translateX(-50%);
    width: min(1240px, calc(100vw - 62px));
    display: grid;
    grid-template-rows: min-content 1fr min-content;
    border-radius: 20px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 30%, transparent);
    background:
      radial-gradient(120% 90% at 8% 0%, color-mix(in oklab, var(--accent-a) 12%, transparent), transparent 56%),
      radial-gradient(110% 100% at 95% 0%, color-mix(in oklab, var(--accent-b) 11%, transparent), transparent 58%),
      linear-gradient(180deg, color-mix(in oklab, #131a32 82%, transparent), color-mix(in oklab, #0b1223 88%, transparent));
    backdrop-filter: blur(16px) saturate(136%);
    box-shadow: 0 22px 54px rgba(3, 8, 18, 0.52), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    overflow: hidden;
    color: #eef5ff;
    z-index: 170;
  }

  .apps-menu-header {
    width: 100%;
    padding: 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    border-bottom: 1px solid color-mix(in oklab, var(--accent-a) 16%, transparent);
  }

  .apps-menu-body {
    width: 100%;
    height: 100%;
    overflow: hidden;
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 10px;
    padding: 10px;
  }

  .apps-menu-footer {
    width: 100%;
    padding: 10px;
    border-top: 1px solid color-mix(in oklab, var(--accent-a) 16%, transparent);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .apps-menu-footer-left,
  .apps-menu-footer-right,
  .header-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  input,
  .apps-menu-header button,
  .app,
  .app-row,
  .stack button,
  .window-row,
  .empty-box,
  .apps-menu-footer button {
    border: 1px solid color-mix(in oklab, var(--accent-a) 22%, transparent);
    border-radius: 10px;
    background: color-mix(in oklab, #edf3ff 7%, transparent);
    color: inherit;
  }

  input {
    flex: 1;
    min-height: 38px;
    padding: 0 12px;
    font-size: 14px;
  }

  .apps-menu-header button,
  .apps-menu-footer button {
    min-height: 34px;
    padding: 0 11px;
    font-size: 12px;
    cursor: pointer;
  }

  .header-actions button.active,
  .stack button.active {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 24%, transparent), color-mix(in oklab, var(--accent-b) 13%, transparent)),
      color-mix(in oklab, #edf3ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 40%, transparent);
  }

  .body-main,
  .body-side {
    min-height: 0;
    border: 1px solid color-mix(in oklab, var(--accent-a) 14%, transparent);
    border-radius: 12px;
    background: color-mix(in oklab, #f2f7ff 3%, transparent);
    padding: 10px;
  }

  .body-main {
    display: grid;
    grid-template-rows: auto auto auto 1fr;
    gap: 8px;
  }

  h3 {
    margin: 0;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: color-mix(in oklab, #eef5ff 58%, transparent);
  }

  .pinned-view-list {
    display: grid;
    grid-template-columns: repeat(var(--columns), 1fr);
    gap: 8px;
  }

  .app {
    display: grid;
    align-items: center;
    justify-items: center;
    gap: 8px;
    padding: 8px;
    border-radius: 10px;
    cursor: pointer;
    min-height: 72px;
  }

  .app-row {
    min-height: 36px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 10px;
    cursor: pointer;
    font-size: 12px;
  }

  .row-main {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .app-icon {
    width: 20px;
    height: 20px;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 700;
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 24%, transparent), color-mix(in oklab, var(--accent-b) 14%, transparent)),
      color-mix(in oklab, #edf3ff 7%, transparent);
  }

  .app-icon.small {
    width: 16px;
    height: 16px;
    font-size: 9px;
  }

  .app-name {
    width: 100%;
    text-align: center;
    font-size: 12px;
    line-height: 1.2em;
    font-weight: 600;
  }

  .all-apps-view-list {
    overflow-y: auto;
    display: grid;
    gap: 7px;
    min-height: 0;
  }

  .body-side {
    display: grid;
    align-content: start;
    gap: 10px;
  }

  .stack {
    display: grid;
    gap: 7px;
  }

  .stack button,
  .window-row,
  .empty-box {
    min-height: 34px;
    display: flex;
    align-items: center;
    padding: 0 10px;
    font-size: 12px;
  }

  .stack button {
    cursor: pointer;
    text-align: left;
  }

  @media (max-width: 1080px) {
    .apps-menu {
      width: calc(100vw - 24px);
      top: 66px;
      bottom: 60px;
    }

    .apps-menu-body {
      grid-template-columns: 1fr;
    }

    .pinned-view-list {
      grid-template-columns: repeat(3, 1fr);
    }

    .apps-menu-footer-right {
      display: none;
    }
  }
</style>
