<script>
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  let { apps, onclose } = $props()

  let searchQuery = $state('')
  let showSettings = $state(false)
  let aiKey = $state('')
  let aiProvider = $state('openai')

  const featuredIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']
  const quickIds = ['terminal', 'explorer', 'settings']

  let featuredApps = $derived(apps.filter((app) => featuredIds.includes(app.id)).slice(0, 6))
  let quickApps = $derived(apps.filter((app) => quickIds.includes(app.id)).slice(0, 3))
  let recentApps = $derived(apps.slice(0, 10))

  let filteredApps = $derived(
    searchQuery.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(searchQuery.toLowerCase())).slice(0, 12)
      : recentApps
  )

  onMount(() => {
    const savedKey = localStorage.getItem('vortex_ai_key')
    const savedProvider = localStorage.getItem('vortex_ai_provider')
    if (savedKey) aiKey = savedKey
    if (savedProvider) aiProvider = savedProvider
  })

  async function handleAppClick(app) {
    try {
      await invoke('open_app', { appId: app.id })
    } catch (error) {
      console.log('Launch error:', error)
    }
    onclose()
  }

  function saveSettings() {
    localStorage.setItem('vortex_ai_key', aiKey)
    localStorage.setItem('vortex_ai_provider', aiProvider)
    showSettings = false
  }
</script>

<div
  class="start-menu"
  onclick={(event) => event.stopPropagation()}
  onkeydown={(event) => event.key === 'Escape' && onclose()}
  role="dialog"
  tabindex="-1"
>
  {#if !showSettings}
    <section class="hero">
      <div class="hero-copy">
        <span class="eyebrow">VORTEX Control Deck</span>
        <h2>Liquid-glass shell launcher</h2>
        <p>Fast launch, app indexing, and shell controls from one focused surface.</p>
      </div>
      <div class="hero-orb"></div>
    </section>

    <section class="search-section">
      <div class="search-box">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input
          type="text"
          placeholder="Search your indexed apps..."
          bind:value={searchQuery}
        />
        <span class="search-hint">Indexed: {apps.length}</span>
      </div>
    </section>

    <section class="content-grid">
      <div class="left-column">
        <div class="panel">
          <div class="section-header">
            <span>Featured Apps</span>
          </div>
          <div class="apps-grid">
            {#each featuredApps as app}
              <button class="app-card" onclick={() => handleAppClick(app)}>
                <div class="app-icon">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z" />
                  </svg>
                </div>
                <span>{app.name}</span>
              </button>
            {/each}
          </div>
        </div>

        <div class="panel">
          <div class="section-header">
            <span>Search Results</span>
          </div>
          <div class="results-list">
            {#each filteredApps as app}
              <button class="result-row" onclick={() => handleAppClick(app)}>
                <div class="result-pill"></div>
                <div class="result-copy">
                  <strong>{app.name}</strong>
                  <span>{app.source}</span>
                </div>
              </button>
            {/each}
          </div>
        </div>
      </div>

      <div class="right-column">
        <div class="panel ambient">
          <div class="section-header">
            <span>Quick Actions</span>
          </div>
          <div class="quick-actions">
            {#each quickApps as app}
              <button class="quick-action" onclick={() => handleAppClick(app)}>
                <span>{app.name}</span>
              </button>
            {/each}
            <button class="quick-action accent" onclick={() => (showSettings = true)}>
              <span>Shell Settings</span>
            </button>
          </div>
        </div>

        <div class="panel ambient">
          <div class="section-header">
            <span>Shell Notes</span>
          </div>
          <div class="metrics">
            <div class="metric-card">
              <span>State</span>
              <strong>Active build</strong>
            </div>
            <div class="metric-card">
              <span>Mode</span>
              <strong>Production path</strong>
            </div>
            <div class="metric-card">
              <span>AI</span>
              <strong>Backend pending</strong>
            </div>
          </div>
        </div>

        <div class="footer-strip">
          <div class="user-badge">
            <div class="user-avatar">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />
              </svg>
            </div>
            <div class="user-copy">
              <strong>VORTEX</strong>
              <span>Personal shell workspace</span>
            </div>
          </div>
          <div class="footer-actions">
            <button class="icon-button" title="Settings" onclick={() => (showSettings = true)}>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.04.31-.06.63-.06.94s.02.63.06.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z" />
              </svg>
            </button>
            <button class="icon-button danger" title="Exit Shell" onclick={() => invoke('close_window').catch((error) => console.log('Close shell error:', error))}>
              <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                <path d="M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </section>
  {:else}
    <div class="settings-view">
      <div class="settings-header">
        <button class="icon-button" aria-label="Back to launcher" onclick={() => (showSettings = false)}>
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="15 18 9 12 15 6"></polyline>
          </svg>
        </button>
        <div class="settings-title">
          <span>System Settings</span>
          <h2>AI provider wiring</h2>
        </div>
      </div>

      <div class="settings-content">
        <div class="setting-group">
          <label for="ai-provider">Provider</label>
          <select id="ai-provider" bind:value={aiProvider} class="modern-input">
            <option value="openai">OpenAI</option>
            <option value="anthropic">Anthropic</option>
            <option value="google">Google</option>
            <option value="local">Local</option>
          </select>

          <label for="ai-key">API Key</label>
          <input id="ai-key" type="password" bind:value={aiKey} placeholder="sk-..." class="modern-input" />

          <p class="setting-desc">These values are saved locally and reserved for the future production AI backend.</p>
          <button class="save-button" onclick={saveSettings}>Save Shell Settings</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .start-menu {
    position: absolute;
    bottom: 132px;
    left: 50%;
    transform: translateX(-50%);
    width: 980px;
    min-height: 620px;
    border-radius: 34px;
    overflow: hidden;
    color: #eef4ff;
    border: 1px solid rgba(255, 255, 255, 0.1);
    background:
      linear-gradient(180deg, rgba(23, 30, 42, 0.96), rgba(10, 14, 20, 0.96)),
      radial-gradient(circle at top right, rgba(95, 154, 255, 0.25), transparent 32%);
    backdrop-filter: blur(36px) saturate(150%);
    box-shadow:
      0 30px 80px rgba(0, 0, 0, 0.45),
      inset 0 1px 0 rgba(255, 255, 255, 0.08);
    z-index: 100;
    animation: riseIn 0.28s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes riseIn {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(22px) scale(0.97);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0) scale(1);
    }
  }

  .hero {
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 28px 30px 18px;
    overflow: hidden;
  }

  .hero-copy {
    display: grid;
    gap: 8px;
    max-width: 520px;
  }

  .eyebrow,
  .section-header span,
  .user-copy span,
  .metric-card span,
  .result-copy span,
  .settings-title span,
  .setting-desc {
    font-size: 11px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: rgba(214, 227, 255, 0.62);
  }

  .hero h2,
  .settings-title h2 {
    margin: 0;
    font-size: 30px;
    line-height: 1.05;
    font-weight: 700;
  }

  .hero p {
    margin: 0;
    color: rgba(228, 238, 255, 0.72);
    max-width: 460px;
    font-size: 14px;
  }

  .hero-orb {
    width: 170px;
    height: 170px;
    border-radius: 999px;
    background:
      radial-gradient(circle at 35% 35%, rgba(255, 255, 255, 0.72), rgba(255, 255, 255, 0.08) 28%, rgba(91, 155, 255, 0.24) 48%, transparent 70%),
      linear-gradient(135deg, rgba(131, 190, 255, 0.32), rgba(0, 0, 0, 0));
    filter: blur(2px);
    opacity: 0.95;
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.35);
  }

  .search-section {
    padding: 0 30px 22px;
  }

  .search-box,
  .panel,
  .footer-strip,
  .modern-input,
  .quick-action,
  .icon-button {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 18px;
    border-radius: 22px;
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
  }

  .search-box input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    color: white;
    font-size: 15px;
  }

  .search-hint {
    font-size: 11px;
    color: rgba(214, 227, 255, 0.55);
  }

  .content-grid {
    display: grid;
    grid-template-columns: 1.4fr 0.9fr;
    gap: 18px;
    padding: 0 30px 28px;
  }

  .left-column,
  .right-column {
    display: grid;
    gap: 18px;
  }

  .panel {
    border-radius: 26px;
    padding: 18px;
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
  }

  .panel.ambient {
    background:
      linear-gradient(180deg, rgba(255, 255, 255, 0.055), rgba(255, 255, 255, 0.03)),
      radial-gradient(circle at top right, rgba(91, 155, 255, 0.18), transparent 40%);
  }

  .section-header {
    margin-bottom: 14px;
  }

  .apps-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 12px;
  }

  .app-card,
  .result-row,
  .quick-action,
  .save-button {
    border: none;
    cursor: pointer;
    color: inherit;
  }

  .app-card {
    display: grid;
    justify-items: center;
    gap: 10px;
    padding: 16px 10px;
    border-radius: 22px;
    background: rgba(255, 255, 255, 0.04);
    transition: transform 0.18s ease, background 0.18s ease;
  }

  .app-card:hover,
  .result-row:hover,
  .quick-action:hover,
  .icon-button:hover,
  .save-button:hover {
    transform: translateY(-1px);
    background: rgba(255, 255, 255, 0.085);
  }

  .app-icon {
    width: 54px;
    height: 54px;
    display: grid;
    place-items: center;
    border-radius: 18px;
    background:
      linear-gradient(135deg, rgba(255, 255, 255, 0.14), rgba(255, 255, 255, 0.04)),
      rgba(91, 155, 255, 0.16);
    color: #a8c9ff;
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.18);
  }

  .app-card span,
  .quick-action span {
    font-size: 13px;
    font-weight: 600;
  }

  .results-list,
  .quick-actions,
  .metrics {
    display: grid;
    gap: 10px;
  }

  .result-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.035);
    text-align: left;
  }

  .result-pill {
    width: 10px;
    height: 36px;
    border-radius: 999px;
    background: linear-gradient(180deg, #a8ccff, #4788ff);
    box-shadow: 0 0 14px rgba(91, 155, 255, 0.34);
    flex: 0 0 auto;
  }

  .result-copy {
    display: grid;
    gap: 3px;
  }

  .result-copy strong,
  .metric-card strong,
  .user-copy strong {
    font-size: 13px;
    font-weight: 700;
  }

  .quick-action {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    border-radius: 18px;
  }

  .quick-action.accent {
    background:
      linear-gradient(135deg, rgba(91, 155, 255, 0.24), rgba(91, 155, 255, 0.08)),
      rgba(255, 255, 255, 0.06);
  }

  .metric-card {
    display: grid;
    gap: 6px;
    padding: 14px 16px;
    border-radius: 18px;
    background: rgba(255, 255, 255, 0.04);
  }

  .footer-strip {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    border-radius: 22px;
  }

  .user-badge,
  .footer-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .user-avatar,
  .icon-button {
    width: 42px;
    height: 42px;
    display: grid;
    place-items: center;
    border-radius: 16px;
  }

  .user-avatar {
    background:
      linear-gradient(135deg, rgba(255, 255, 255, 0.12), rgba(255, 255, 255, 0.02)),
      rgba(91, 155, 255, 0.2);
  }

  .user-copy {
    display: grid;
    gap: 3px;
  }

  .icon-button {
    color: rgba(232, 240, 255, 0.86);
  }

  .icon-button.danger {
    color: #ffb4b1;
  }

  .settings-view {
    display: grid;
    gap: 20px;
    padding: 28px 30px 30px;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .settings-title {
    display: grid;
    gap: 6px;
  }

  .setting-group {
    display: grid;
    gap: 12px;
    padding: 22px;
    border-radius: 24px;
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .setting-group label {
    font-size: 12px;
    color: rgba(227, 237, 255, 0.72);
  }

  .modern-input {
    padding: 14px 16px;
    border-radius: 18px;
    color: white;
    outline: none;
  }

  .modern-input option {
    background: #10141c;
    color: white;
  }

  .save-button {
    margin-top: 8px;
    padding: 14px 16px;
    border-radius: 18px;
    background:
      linear-gradient(135deg, rgba(91, 155, 255, 0.34), rgba(91, 155, 255, 0.14)),
      rgba(255, 255, 255, 0.05);
    font-weight: 700;
  }
</style>
