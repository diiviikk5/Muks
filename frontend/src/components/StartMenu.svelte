<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { apps, onclose } = $props()

  let query = $state('')
  let tab = $state('launch')

  const featuredIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']
  const quickIds = ['terminal', 'explorer', 'settings']

  let featuredApps = $derived(apps.filter((app) => featuredIds.includes(app.id)).slice(0, 6))
  let quickApps = $derived(apps.filter((app) => quickIds.includes(app.id)).slice(0, 3))

  let filteredApps = $derived(
    query.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 20)
      : apps.slice(0, 20)
  )

  onMount(() => {
    const onKey = (event) => {
      if (event.key === 'Escape') onclose()
    }
    window.addEventListener('keydown', onKey)
    return () => window.removeEventListener('keydown', onKey)
  })

  async function launch(app) {
    try {
      await invoke('apps_launch', { appId: app.id })
    } catch (error) {
      console.log('Launch error:', error)
    }
    onclose()
  }
</script>

<div
  class="menu-shell"
  onclick={(event) => event.stopPropagation()}
  onkeydown={(event) => event.key === 'Escape' && onclose()}
  role="dialog"
  tabindex="-1"
>
  <div class="menu-header">
    <div>
      <span class="kicker">Vortex Launcher</span>
      <h2>Find anything instantly</h2>
    </div>
    <button class="close-btn" onclick={onclose} aria-label="Close menu">x</button>
  </div>

  <div class="search-row">
    <input type="text" placeholder="Type app name..." bind:value={query} />
    <button onclick={() => (tab = 'launch')} class:active={tab === 'launch'}>Apps</button>
    <button onclick={() => (tab = 'widgets')} class:active={tab === 'widgets'}>Widgets</button>
    <button onclick={() => (tab = 'settings')} class:active={tab === 'settings'}>Settings</button>
  </div>

  <div class="menu-grid">
    <section class="panel">
      <div class="panel-title">Featured</div>
      <div class="tiles">
        {#each featuredApps as app}
          <button class="tile" onclick={() => launch(app)}>
            <span class="tile-dot"></span>
            <span>{app.name}</span>
          </button>
        {/each}
      </div>

      <div class="panel-title">Results</div>
      <div class="list">
        {#each filteredApps as app}
          <button class="list-item" onclick={() => launch(app)}>
            <span>{app.name}</span>
            <small>{app.source}</small>
          </button>
        {/each}
      </div>
    </section>

    <aside class="panel side">
      <div class="panel-title">Quick Actions</div>
      <div class="actions">
        {#each quickApps as app}
          <button onclick={() => launch(app)}>{app.name}</button>
        {/each}
      </div>

      <div class="panel-title">Workspace</div>
      <div class="meta">
        <div><span>Indexed apps</span><strong>{apps.length}</strong></div>
        <div><span>Mode</span><strong>Custom build</strong></div>
        <div><span>Status</span><strong>Live runtime</strong></div>
      </div>
    </aside>
  </div>
</div>

<style>
  .menu-shell {
    position: absolute;
    left: 50%;
    bottom: 118px;
    transform: translateX(-50%);
    width: min(1040px, calc(100vw - 24px));
    max-height: 72vh;
    overflow: hidden;
    border-radius: 28px;
    border: 1px solid rgba(163, 190, 255, 0.24);
    background: rgba(11, 17, 27, 0.84);
    backdrop-filter: blur(18px) saturate(128%);
    box-shadow: 0 30px 70px rgba(0, 0, 0, 0.48);
    color: #e7f0ff;
    z-index: 120;
  }

  .menu-header,
  .search-row {
    padding: 16px 18px;
    display: flex;
    align-items: center;
    gap: 10px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .menu-header {
    justify-content: space-between;
  }

  .kicker {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(214, 227, 255, 0.62);
  }

  h2 {
    margin: 4px 0 0;
    font-size: 26px;
    line-height: 1;
  }

  .close-btn,
  .search-row button,
  .tile,
  .list-item,
  .actions button {
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.06);
    color: inherit;
    border-radius: 12px;
    cursor: pointer;
  }

  .close-btn {
    width: 30px;
    height: 30px;
  }

  .search-row input {
    flex: 1;
    height: 38px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.04);
    color: #f2f7ff;
    padding: 0 12px;
  }

  .search-row button {
    height: 38px;
    padding: 0 12px;
  }

  .search-row button.active {
    background: rgba(102, 167, 255, 0.26);
    border-color: rgba(102, 167, 255, 0.48);
  }

  .menu-grid {
    display: grid;
    grid-template-columns: 1.5fr 0.8fr;
    gap: 12px;
    padding: 14px;
  }

  .panel {
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.03);
    padding: 12px;
    min-height: 0;
  }

  .panel-title {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(214, 227, 255, 0.62);
    margin-bottom: 10px;
  }

  .tiles {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 8px;
    margin-bottom: 12px;
  }

  .tile {
    height: 38px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 10px;
    text-align: left;
  }

  .tile-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #7eb8ff;
    box-shadow: 0 0 8px rgba(102, 167, 255, 0.52);
  }

  .list {
    display: grid;
    gap: 6px;
    max-height: 320px;
    overflow-y: auto;
  }

  .list-item {
    min-height: 40px;
    padding: 0 10px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    text-align: left;
    font-size: 13px;
  }

  .list-item small {
    color: rgba(218, 231, 255, 0.62);
    font-size: 11px;
  }

  .actions {
    display: grid;
    gap: 8px;
    margin-bottom: 16px;
  }

  .actions button {
    height: 36px;
    text-align: left;
    padding: 0 10px;
  }

  .meta {
    display: grid;
    gap: 8px;
  }

  .meta div {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 10px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.04);
    font-size: 12px;
  }

  .meta span {
    color: rgba(214, 227, 255, 0.66);
  }
</style>
