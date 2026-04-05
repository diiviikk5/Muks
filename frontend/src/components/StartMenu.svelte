<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, windows = [], activePreset, onSelectPreset, onclose } = $props()
  let query = $state('')

  const featuredIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let featuredApps = $derived(apps.filter((app) => featuredIds.includes(app.id)).slice(0, 6))
  let filteredApps = $derived(
    query.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 28)
      : apps.slice(0, 28)
  )
  let liveWindows = $derived(windows.slice(0, 4))

  async function launch(app) {
    try {
      await invoke('apps_launch', { appId: app.id })
    } catch (error) {
      console.log('Launch error:', error)
    }
    onclose()
  }
</script>

<div class="menu-shell" onclick={(event) => event.stopPropagation()} onkeydown={(event) => event.key === 'Escape' && onclose()} role="dialog" tabindex="-1" aria-label="Launcher">
  <header>
    <div class="title-wrap">
      <span class="eyebrow">Vortex Launcher</span>
      <h2>Everything, right now</h2>
      <p>Low-latency launch, active window control, and profile-ready layouts.</p>
    </div>
    <button class="close" onclick={onclose}>x</button>
  </header>

  <div class="search-wrap">
    <input type="text" placeholder="Type to launch apps or jump into a workflow" bind:value={query} />
    <div class="hints">
      <kbd>Alt</kbd>
      <span>+</span>
      <kbd>Space</kbd>
      <small>Palette</small>
    </div>
  </div>

  <div class="content">
    <section class="panel stage">
      <h3>Featured</h3>
      <div class="hero-grid">
        {#each featuredApps as app}
          <button class="hero-card" onclick={() => launch(app)}>
            <span class="seed"></span>
            <strong>{app.name}</strong>
            <small>Open instantly</small>
          </button>
        {/each}
      </div>

      <h3>Catalog</h3>
      <div class="catalog-list">
        {#each filteredApps as app}
          <button class="catalog-row" onclick={() => launch(app)}>
            <span class="row-left">
              <span class="seed small"></span>
              <span>{app.name}</span>
            </span>
            <small>{app.source}</small>
          </button>
        {/each}
      </div>
    </section>

    <aside class="panel side">
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
            <div class="muted">No active windows</div>
          {:else}
            {#each liveWindows as item}
              <div class="window-row">
                <span>{item.title}</span>
              </div>
            {/each}
          {/if}
        </div>
      </section>

      <section>
        <h3>Status</h3>
        <div class="metrics">
          <div>
            <span>Indexed</span>
            <strong>{apps.length}</strong>
          </div>
          <div>
            <span>Theme</span>
            <strong>Nebula</strong>
          </div>
          <div>
            <span>Engine</span>
            <strong>Rust Live</strong>
          </div>
        </div>
      </section>
    </aside>
  </div>
</div>

<style>
  .menu-shell {
    position: absolute;
    left: 50%;
    bottom: 128px;
    transform: translateX(-50%);
    width: min(1220px, calc(100vw - 30px));
    max-height: min(78vh, 920px);
    border-radius: 30px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 34%, transparent);
    background:
      radial-gradient(120% 90% at 8% 0%, color-mix(in oklab, var(--accent-a) 14%, transparent), transparent 58%),
      radial-gradient(110% 100% at 95% 0%, color-mix(in oklab, var(--accent-b) 12%, transparent), transparent 60%),
      linear-gradient(180deg, color-mix(in oklab, #141d38 74%, transparent), color-mix(in oklab, #0d1329 78%, transparent));
    box-shadow: 0 40px 90px rgba(2, 8, 18, 0.62), inset 0 1px 0 rgba(255, 255, 255, 0.09);
    backdrop-filter: blur(20px) saturate(145%);
    color: #eef5ff;
    z-index: 170;
    overflow: hidden;
    animation: rise 180ms ease;
  }

  header,
  .search-wrap {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 16px 18px;
    border-bottom: 1px solid color-mix(in oklab, #bdd1ff 16%, transparent);
  }

  .eyebrow,
  h3 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: color-mix(in oklab, #eef5ff 58%, transparent);
  }

  .title-wrap h2 {
    margin: 6px 0 0;
    font-size: 38px;
    line-height: 0.96;
    letter-spacing: -0.03em;
  }

  .title-wrap p {
    margin: 10px 0 0;
    color: color-mix(in oklab, #f2f7ff 67%, transparent);
    font-size: 14px;
  }

  .close,
  input,
  .hero-card,
  .catalog-row,
  .stack button,
  .window-row,
  .metrics div,
  .muted {
    border: 1px solid color-mix(in oklab, var(--accent-a) 22%, transparent);
    border-radius: 14px;
    background: color-mix(in oklab, #edf3ff 7%, transparent);
    color: inherit;
  }

  .close {
    width: 36px;
    height: 36px;
    border-radius: 999px;
    cursor: pointer;
    transition: transform 150ms ease, background-color 150ms ease;
  }

  .close:hover,
  .hero-card:hover,
  .catalog-row:hover,
  .stack button:hover {
    transform: translateY(-1px);
    background: color-mix(in oklab, #edf3ff 14%, transparent);
  }

  input {
    width: 100%;
    min-height: 46px;
    padding: 0 16px;
    font-size: 15px;
    letter-spacing: 0.02em;
    background: color-mix(in oklab, #edf3ff 9%, transparent);
  }

  .hints {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
    color: color-mix(in oklab, #eff6ff 70%, transparent);
    font-size: 11px;
  }

  kbd {
    min-width: 24px;
    height: 24px;
    padding: 0 8px;
    border-radius: 8px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 28%, transparent);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in oklab, #edf3ff 10%, transparent);
    font-size: 11px;
    font-family: 'Consolas', 'JetBrains Mono', monospace;
  }

  .content {
    padding: 14px;
    display: grid;
    grid-template-columns: minmax(0, 1.7fr) minmax(280px, 0.9fr);
    gap: 14px;
    min-height: 0;
  }

  .panel {
    border-radius: 20px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 16%, transparent);
    background: color-mix(in oklab, #f2f7ff 4%, transparent);
    padding: 14px;
    min-height: 0;
  }

  h3 {
    margin: 0 0 10px;
  }

  .hero-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 10px;
    margin-bottom: 16px;
  }

  .hero-card {
    min-height: 72px;
    padding: 10px 12px;
    display: grid;
    gap: 4px;
    text-align: left;
    cursor: pointer;
    transition: transform 150ms ease, background-color 150ms ease;
  }

  .hero-card strong {
    font-size: 14px;
    letter-spacing: 0.01em;
  }

  .hero-card small {
    color: color-mix(in oklab, #eef5ff 62%, transparent);
    font-size: 11px;
  }

  .seed {
    width: 10px;
    height: 10px;
    border-radius: 99px;
    background: color-mix(in oklab, var(--accent-a) 90%, white 10%);
    box-shadow: 0 0 16px color-mix(in oklab, var(--accent-a) 60%, transparent);
  }

  .seed.small {
    width: 8px;
    height: 8px;
  }

  .catalog-list {
    display: grid;
    gap: 8px;
    max-height: 365px;
    overflow-y: auto;
    scrollbar-width: thin;
  }

  .catalog-row {
    min-height: 40px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: transform 150ms ease, background-color 150ms ease;
    font-size: 13px;
  }

  .row-left {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
    overflow: hidden;
  }

  .catalog-row small {
    color: color-mix(in oklab, #f0f6ff 52%, transparent);
    font-size: 11px;
    text-transform: lowercase;
  }

  .side {
    display: grid;
    align-content: start;
    gap: 14px;
  }

  .stack {
    display: grid;
    gap: 8px;
  }

  .stack button,
  .window-row,
  .muted {
    min-height: 38px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    font-size: 13px;
  }

  .stack button {
    cursor: pointer;
    text-align: left;
    transition: transform 150ms ease, background-color 150ms ease;
  }

  .stack button.active {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 26%, transparent), color-mix(in oklab, var(--accent-b) 14%, transparent)),
      color-mix(in oklab, #edf3ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 44%, transparent);
  }

  .window-row {
    color: color-mix(in oklab, #f0f6ff 76%, transparent);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .muted {
    color: color-mix(in oklab, #f0f6ff 58%, transparent);
  }

  .metrics {
    display: grid;
    gap: 8px;
  }

  .metrics div {
    min-height: 38px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
  }

  .metrics span {
    color: color-mix(in oklab, #eef5ff 58%, transparent);
  }

  .metrics strong {
    font-size: 12px;
    font-weight: 650;
    letter-spacing: 0.03em;
  }

  @media (max-width: 980px) {
    .menu-shell {
      bottom: 114px;
      max-height: calc(100vh - 148px);
    }

    .title-wrap h2 {
      font-size: 30px;
    }

    .content {
      grid-template-columns: 1fr;
    }

    .hero-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(8px) scale(0.99);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0) scale(1);
    }
  }
</style>
