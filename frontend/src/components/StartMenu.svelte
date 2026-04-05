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
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 20)
      : apps.slice(0, 20)
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
  <div class="menu-header">
    <div>
      <span class="kicker">Launcher</span>
      <h2>Desktop Control</h2>
    </div>
    <button class="close-btn" onclick={onclose} aria-label="Close menu">x</button>
  </div>

  <div class="search-row">
    <input type="text" placeholder="Search apps" bind:value={query} />
  </div>

  <div class="menu-grid">
    <section class="panel left">
      <div class="panel-title">Featured</div>
      <div class="tiles">
        {#each featuredApps as app}
          <button class="tile" onclick={() => launch(app)}>
            <span class="dot"></span>
            <span>{app.name}</span>
          </button>
        {/each}
      </div>

      <div class="panel-title">All Apps</div>
      <div class="list">
        {#each filteredApps as app}
          <button class="list-item" onclick={() => launch(app)}>
            <span>{app.name}</span>
            <small>{app.source}</small>
          </button>
        {/each}
      </div>
    </section>

    <aside class="panel right">
      <div class="panel-title">Quick</div>
      <div class="quick">
        {#each quickApps as app}
          <button onclick={() => launch(app)}>{app.name}</button>
        {/each}
      </div>

      <div class="panel-title">Profiles</div>
      <div class="presets">
        <button>Minimal Glass</button>
        <button>Cyber Night</button>
        <button>Fluent Calm</button>
      </div>
    </aside>
  </div>
</div>

<style>
  .menu-shell {
    position: absolute;
    left: 50%;
    bottom: 102px;
    transform: translateX(-50%);
    width: min(1020px, calc(100vw - 20px));
    max-height: 74vh;
    overflow: hidden;
    border-radius: 26px;
    border: 1px solid rgba(169, 191, 255, 0.24);
    background: rgba(26, 29, 49, 0.84);
    backdrop-filter: blur(16px) saturate(126%);
    box-shadow: 0 30px 70px rgba(0, 0, 0, 0.5);
    color: #e9edff;
    z-index: 120;
  }

  .menu-header,
  .search-row {
    padding: 12px 14px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .kicker,
  .panel-title {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(208, 218, 248, 0.66);
  }

  h2 {
    margin: 3px 0 0;
    font-size: 24px;
    line-height: 1;
  }

  .close-btn,
  input,
  .tile,
  .list-item,
  .quick button,
  .presets button {
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 11px;
    background: rgba(255, 255, 255, 0.06);
    color: inherit;
  }

  .close-btn {
    width: 28px;
    height: 28px;
    cursor: pointer;
  }

  input {
    width: 100%;
    height: 38px;
    padding: 0 12px;
  }

  .menu-grid {
    display: grid;
    grid-template-columns: 1.45fr 0.8fr;
    gap: 10px;
    padding: 10px;
  }

  .panel {
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.03);
    padding: 10px;
  }

  .tiles {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 7px;
    margin: 8px 0 12px;
  }

  .tile,
  .list-item,
  .quick button,
  .presets button {
    cursor: pointer;
  }

  .tile {
    height: 34px;
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 0 9px;
    text-align: left;
    font-size: 12px;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #7ec8ff;
  }

  .list {
    display: grid;
    gap: 6px;
    max-height: 300px;
    overflow-y: auto;
  }

  .list-item {
    min-height: 36px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 9px;
    font-size: 12px;
  }

  .list-item small {
    color: rgba(208, 218, 248, 0.62);
    font-size: 10px;
  }

  .quick,
  .presets {
    display: grid;
    gap: 7px;
    margin: 8px 0 12px;
  }

  .quick button,
  .presets button {
    height: 34px;
    text-align: left;
    padding: 0 10px;
    font-size: 12px;
  }
</style>
