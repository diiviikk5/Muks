<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, onclose } = $props()
  let query = $state('')

  const featuredIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']
  const quickIds = ['terminal', 'explorer', 'settings']

  let featuredApps = $derived(apps.filter((app) => featuredIds.includes(app.id)).slice(0, 6))
  let quickApps = $derived(apps.filter((app) => quickIds.includes(app.id)).slice(0, 3))
  let filteredApps = $derived(
    query.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 24)
      : apps.slice(0, 24)
  )

  async function launch(app) {
    try {
      await invoke('apps_launch', { appId: app.id })
    } catch (error) {
      console.log('Launch error:', error)
    }
    onclose()
  }
</script>

<div class="menu-shell" onclick={(event) => event.stopPropagation()} onkeydown={(event) => event.key === 'Escape' && onclose()} role="dialog" tabindex="-1">
  <header>
    <div>
      <span class="kicker">Launcher</span>
      <h2>Desktop Command Center</h2>
    </div>
    <button class="close" onclick={onclose}>x</button>
  </header>

  <div class="search-wrap">
    <input type="text" placeholder="Search apps or workflows" bind:value={query} />
  </div>

  <div class="content">
    <section class="panel main">
      <h3>Featured</h3>
      <div class="tiles">
        {#each featuredApps as app}
          <button class="tile" onclick={() => launch(app)}>
            <span class="dot"></span>
            <span>{app.name}</span>
          </button>
        {/each}
      </div>

      <h3>Catalog</h3>
      <div class="list">
        {#each filteredApps as app}
          <button class="row" onclick={() => launch(app)}>
            <span>{app.name}</span>
            <small>{app.source}</small>
          </button>
        {/each}
      </div>
    </section>

    <aside class="panel side">
      <h3>Quick</h3>
      <div class="stack">
        {#each quickApps as app}
          <button onclick={() => launch(app)}>{app.name}</button>
        {/each}
      </div>

      <h3>Profiles</h3>
      <div class="stack">
        <button>Nebula</button>
        <button>Fluent Glass</button>
        <button>Studio Dark</button>
      </div>

      <h3>Status</h3>
      <div class="stats">
        <div><span>Indexed</span><strong>{apps.length}</strong></div>
        <div><span>Runtime</span><strong>Live</strong></div>
      </div>
    </aside>
  </div>
</div>

<style>
  .menu-shell {
    position: absolute;
    left: 50%;
    bottom: 100px;
    transform: translateX(-50%);
    width: min(1040px, calc(100vw - 20px));
    max-height: 74vh;
    border-radius: 26px;
    border: 1px solid rgba(174, 196, 255, 0.24);
    background: rgba(29, 33, 56, 0.84);
    backdrop-filter: blur(16px) saturate(126%);
    box-shadow: 0 28px 68px rgba(0, 0, 0, 0.52);
    color: #ebefff;
    z-index: 120;
    overflow: hidden;
  }

  header,
  .search-wrap {
    padding: 12px 14px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .kicker,
  h3 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(207, 217, 247, 0.66);
  }

  h2 {
    margin: 3px 0 0;
    font-size: 24px;
  }

  .close,
  input,
  .tile,
  .row,
  .stack button {
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 11px;
    background: rgba(255, 255, 255, 0.06);
    color: inherit;
  }

  .close {
    width: 28px;
    height: 28px;
    cursor: pointer;
  }

  input {
    width: 100%;
    height: 38px;
    padding: 0 12px;
  }

  .content {
    padding: 10px;
    display: grid;
    grid-template-columns: 1.5fr 0.8fr;
    gap: 10px;
  }

  .panel {
    border-radius: 14px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.03);
    padding: 10px;
    min-height: 0;
  }

  h3 {
    margin: 0 0 8px;
  }

  .tiles {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 7px;
    margin-bottom: 12px;
  }

  .tile {
    height: 34px;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 9px;
    text-align: left;
    cursor: pointer;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #7fc8ff;
  }

  .list {
    display: grid;
    gap: 6px;
    max-height: 310px;
    overflow-y: auto;
  }

  .row {
    min-height: 36px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 10px;
    font-size: 12px;
    cursor: pointer;
  }

  .row small {
    font-size: 10px;
    color: rgba(207, 217, 247, 0.64);
  }

  .stack {
    display: grid;
    gap: 7px;
    margin-bottom: 12px;
  }

  .stack button {
    height: 34px;
    text-align: left;
    padding: 0 10px;
    font-size: 12px;
    cursor: pointer;
  }

  .stats {
    display: grid;
    gap: 7px;
  }

  .stats div {
    height: 34px;
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.09);
    background: rgba(255, 255, 255, 0.05);
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
  }

  .stats span {
    color: rgba(207, 217, 247, 0.68);
  }
</style>
