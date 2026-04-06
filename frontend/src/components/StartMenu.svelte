<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, windows = [], onclose, onSelectPreset, activePreset } = $props()

  let query = $state('')
  let showFavorites = $state(true)

  const favoriteIds = ['explorer', 'browser', 'terminal', 'code', 'settings', 'notepad']

  let favorites = $derived(apps.filter((app) => favoriteIds.includes(app.id)).slice(0, 21))
  let visibleApps = $derived(
    query.trim()
      ? apps.filter((app) => app.name.toLowerCase().includes(query.toLowerCase())).slice(0, 240)
      : apps.slice(0, 240)
  )

  function firstChar(value) {
    const text = (value || '').trim()
    return text ? text[0].toUpperCase() : '?'
  }

  async function launch(app) {
    try {
      await invoke('apps_launch', { appId: app.id })
    } catch {}
    onclose()
  }
</script>

<div class="apps-menu" data-acrylic="true" role="dialog" tabindex="-1" onclick={(event) => event.stopPropagation()} onkeydown={(event) => event.key === 'Escape' && onclose()}>
  <div class="apps-menu-header">
    <input type="search" data-skin="transparent" placeholder="Applications" bind:value={query} />

    <button
      data-skin="default"
      onclick={() => {
        showFavorites = !showFavorites
        query = ''
      }}
    >
      {showFavorites ? 'All' : 'Back'}
    </button>
  </div>

  <div class="apps-menu-body">
    {#if showFavorites}
      <div class="pinned-view">
        <div class="pinned-view-list">
          {#each favorites as app}
            <button class="app" onclick={() => launch(app)}>
              <div class="app-icon" data-shape="square">{firstChar(app.name)}</div>
              <div class="app-name">{app.name}</div>
            </button>
          {/each}
        </div>
      </div>
    {:else}
      <div class="all-apps-view">
        <div class="all-apps-view-list">
          {#each visibleApps as app}
            <button class="app" onclick={() => launch(app)}>
              <div class="app-icon" data-shape="square">{firstChar(app.name)}</div>
              <div class="app-name">{app.name}</div>
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <div class="apps-menu-footer">
    <div class="apps-menu-footer-left">
      <button data-skin="transparent" class="user-profile">
        <span class="user-profile-picture">V</span>
        <span>Vortex User</span>
      </button>
    </div>

    <div class="apps-menu-footer-right">
      <button data-skin="transparent" onclick={() => onSelectPreset('graphite-flare')} class:active={activePreset === 'graphite-flare'}>Graphite</button>
      <button data-skin="transparent" onclick={() => onSelectPreset('forest-glass')} class:active={activePreset === 'forest-glass'}>Forest</button>
      <button data-skin="transparent" onclick={() => onSelectPreset('rose-dusk')} class:active={activePreset === 'rose-dusk'}>Rose</button>
      <button data-skin="transparent" onclick={onclose}>Close</button>
    </div>
  </div>
</div>

<style>
  .apps-menu {
    position: fixed;
    inset: 0;
    z-index: 180;
  }

  .user-profile-picture {
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8rem;
    background: var(--color-gray-300);
    color: var(--color-gray-900);
  }

  .active {
    background-color: var(--slu-std-ui-hover-overlay);
  }
</style>
