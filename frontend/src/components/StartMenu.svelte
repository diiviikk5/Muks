<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, windows = [], activePreset, onSelectPreset, onclose } = $props()
  let query = $state('')
  let viewMode = $state('featured')

  const featuredIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let featuredApps = $derived(apps.filter((app) => featuredIds.includes(app.id)).slice(0, 8))
  let filteredApps = $derived(
    query.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 60)
      : apps.slice(0, 60)
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

  function initials(name) {
    return (name || '?').slice(0, 1).toUpperCase()
  }
</script>

<div class="menu-shell" onclick={(event) => event.stopPropagation()} onkeydown={(event) => event.key === 'Escape' && onclose()} role="dialog" tabindex="-1" aria-label="Launcher">
  <header>
    <div class="title-wrap">
      <span class="eyebrow">Vortex Launcher</span>
      <h2>Everything, right now</h2>
      <p>Fast launch, quick context, no clutter.</p>
    </div>
    <button class="close" onclick={onclose}>x</button>
  </header>

  <div class="search-wrap">
    <input type="text" placeholder="Search apps" bind:value={query} />
    <div class="mode-switch" role="tablist" aria-label="Launcher view">
      <button class:active={viewMode === 'featured'} onclick={() => (viewMode = 'featured')}>Pinned</button>
      <button class:active={viewMode === 'all'} onclick={() => (viewMode = 'all')}>All</button>
    </div>
    <div class="hints">
      <kbd>Alt</kbd>
      <span>+</span>
      <kbd>Space</kbd>
    </div>
  </div>

  <div class="content">
    <section class="panel stage">
      {#if viewMode === 'featured'}
        <h3>Pinned</h3>
        <div class="hero-grid">
          {#each featuredApps as app}
            <button class="hero-card" onclick={() => launch(app)}>
              <span class="seed">{initials(app.name)}</span>
              <div>
                <strong>{app.name}</strong>
                <small>Open instantly</small>
              </div>
            </button>
          {/each}
        </div>

        <h3>Catalog</h3>
      {:else}
        <h3>All Apps</h3>
      {/if}

      <div class="catalog-list">
        {#each filteredApps as app}
          <button class="catalog-row" onclick={() => launch(app)}>
            <span class="row-left">
              <span class="seed small">{initials(app.name)}</span>
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
            <strong>{activePreset}</strong>
          </div>
        </div>
      </section>
    </aside>
  </div>

  <footer>
    <div class="footer-left">
      <button class="footer-pill">diviikk5</button>
      <button class="footer-pill">Workspace A</button>
    </div>
    <div class="footer-right">
      <button class="footer-icon">Settings</button>
      <button class="footer-icon">Power</button>
      <button class="footer-icon">Expand</button>
    </div>
  </footer>
</div>

<style>
  .menu-shell {
    position: absolute;
    left: 50%;
    top: 74px;
    bottom: 78px;
    transform: translateX(-50%);
    width: min(1240px, calc(100vw - 72px));
    border-radius: 24px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 32%, transparent);
    background:
      radial-gradient(120% 90% at 8% 0%, color-mix(in oklab, var(--accent-a) 12%, transparent), transparent 56%),
      radial-gradient(110% 100% at 95% 0%, color-mix(in oklab, var(--accent-b) 11%, transparent), transparent 58%),
      linear-gradient(180deg, color-mix(in oklab, #131a32 78%, transparent), color-mix(in oklab, #0b1223 84%, transparent));
    box-shadow: 0 30px 72px rgba(3, 8, 18, 0.58), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(16px) saturate(138%);
    color: #eef5ff;
    z-index: 170;
    overflow: hidden;
    display: grid;
    grid-template-rows: auto auto 1fr auto;
    animation: rise 160ms ease;
  }

  header,
  .search-wrap {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 12px 14px;
    border-bottom: 1px solid color-mix(in oklab, var(--accent-a) 14%, transparent);
  }

  .eyebrow,
  h3 {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: color-mix(in oklab, #eef5ff 58%, transparent);
  }

  .title-wrap h2 {
    margin: 4px 0 0;
    font-size: 26px;
    line-height: 1.05;
    letter-spacing: -0.02em;
  }

  .title-wrap p {
    margin: 6px 0 0;
    font-size: 13px;
    color: color-mix(in oklab, #f0f6ff 64%, transparent);
  }

  .close,
  input,
  .hero-card,
  .catalog-row,
  .stack button,
  .window-row,
  .metrics div,
  .muted,
  .mode-switch button,
  .seed {
    border: 1px solid color-mix(in oklab, var(--accent-a) 22%, transparent);
    border-radius: 12px;
    background: color-mix(in oklab, #edf3ff 6%, transparent);
    color: inherit;
  }

  .close {
    width: 34px;
    height: 34px;
    border-radius: 999px;
    cursor: pointer;
    transition: transform 120ms ease, background-color 120ms ease;
  }

  .close:hover,
  .hero-card:hover,
  .catalog-row:hover,
  .stack button:hover,
  .mode-switch button:hover {
    transform: translateY(-1px);
    background: color-mix(in oklab, #edf3ff 12%, transparent);
  }

  input {
    width: 100%;
    min-height: 40px;
    padding: 0 13px;
    font-size: 14px;
  }

  .mode-switch {
    display: inline-flex;
    gap: 6px;
    padding: 3px;
    border-radius: 10px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 24%, transparent);
    background: color-mix(in oklab, #edf3ff 4%, transparent);
  }

  .mode-switch button {
    min-height: 28px;
    padding: 0 10px;
    border-radius: 8px;
    font-size: 12px;
    cursor: pointer;
    border-color: transparent;
    transition: transform 120ms ease, background-color 120ms ease;
  }

  .mode-switch button.active {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 22%, transparent), color-mix(in oklab, var(--accent-b) 13%, transparent)),
      color-mix(in oklab, #edf3ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 40%, transparent);
  }

  .hints {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: color-mix(in oklab, #eff6ff 68%, transparent);
  }

  kbd {
    min-width: 23px;
    height: 23px;
    padding: 0 8px;
    border-radius: 7px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 28%, transparent);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in oklab, #edf3ff 10%, transparent);
    font-size: 11px;
    font-family: 'Consolas', 'JetBrains Mono', monospace;
  }

  .content {
    min-height: 0;
    padding: 12px;
    display: grid;
    grid-template-columns: minmax(0, 1.85fr) minmax(280px, 0.85fr);
    gap: 12px;
  }

  footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 10px 12px;
    border-top: 1px solid color-mix(in oklab, var(--accent-a) 14%, transparent);
  }

  .footer-left,
  .footer-right {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .footer-pill,
  .footer-icon {
    min-height: 32px;
    padding: 0 11px;
    border-radius: 10px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 22%, transparent);
    background: color-mix(in oklab, #edf3ff 6%, transparent);
    color: color-mix(in oklab, #f0f6ff 82%, transparent);
    font-size: 12px;
    cursor: pointer;
    transition: transform 120ms ease, background-color 120ms ease;
  }

  .footer-pill:hover,
  .footer-icon:hover {
    transform: translateY(-1px);
    background: color-mix(in oklab, #edf3ff 12%, transparent);
  }

  .panel {
    min-height: 0;
    border-radius: 16px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 14%, transparent);
    background: color-mix(in oklab, #f2f7ff 3%, transparent);
    padding: 10px;
  }

  .stage {
    display: grid;
    grid-template-rows: auto auto auto 1fr;
    gap: 8px;
  }

  h3 {
    margin: 0;
  }

  .hero-grid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
    margin-bottom: 2px;
  }

  .hero-card {
    min-height: 58px;
    padding: 8px 10px;
    display: flex;
    align-items: center;
    gap: 10px;
    text-align: left;
    cursor: pointer;
    transition: transform 120ms ease, background-color 120ms ease;
  }

  .hero-card strong {
    display: block;
    font-size: 13px;
    line-height: 1.2;
  }

  .hero-card small {
    color: color-mix(in oklab, #eff5ff 60%, transparent);
    font-size: 11px;
  }

  .seed {
    width: 20px;
    height: 20px;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 600;
    flex-shrink: 0;
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 28%, transparent), color-mix(in oklab, var(--accent-b) 16%, transparent)),
      color-mix(in oklab, #edf3ff 7%, transparent);
  }

  .seed.small {
    width: 16px;
    height: 16px;
    font-size: 9px;
  }

  .catalog-list {
    min-height: 0;
    display: grid;
    gap: 7px;
    overflow-y: auto;
    padding-right: 2px;
  }

  .catalog-row {
    min-height: 36px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    font-size: 13px;
    transition: transform 120ms ease, background-color 120ms ease;
  }

  .row-left {
    display: inline-flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .catalog-row small {
    font-size: 10px;
    color: color-mix(in oklab, #f0f6ff 50%, transparent);
    text-transform: lowercase;
  }

  .side {
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
  .muted {
    min-height: 34px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    font-size: 12px;
  }

  .stack button {
    cursor: pointer;
    text-align: left;
    transition: transform 120ms ease, background-color 120ms ease;
  }

  .stack button.active {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 24%, transparent), color-mix(in oklab, var(--accent-b) 13%, transparent)),
      color-mix(in oklab, #edf3ff 7%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 42%, transparent);
  }

  .window-row {
    color: color-mix(in oklab, #f0f6ff 75%, transparent);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .muted {
    color: color-mix(in oklab, #f0f6ff 58%, transparent);
  }

  .metrics {
    display: grid;
    gap: 7px;
  }

  .metrics div {
    min-height: 34px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
  }

  .metrics span {
    color: color-mix(in oklab, #eef5ff 58%, transparent);
  }

  .metrics strong {
    font-size: 11px;
    font-weight: 650;
    letter-spacing: 0.02em;
    text-transform: capitalize;
  }

  @media (max-width: 1100px) {
    .menu-shell {
      width: calc(100vw - 26px);
      top: 68px;
      bottom: 68px;
    }

    .content {
      grid-template-columns: 1fr;
    }

    .hero-grid {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }

    .hints {
      display: none;
    }
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(6px) scale(0.995);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0) scale(1);
    }
  }
</style>
