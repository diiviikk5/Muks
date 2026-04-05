<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, onclose, onExecute } = $props()

  let query = $state('')
  let selected = $state(0)
  let backendResults = $state([])
  let inputRef = $state()
  let scope = $state('all')

  let commands = $derived([
    ...apps.map((app) => ({ id: app.id, name: app.name, type: 'app', group: 'Apps' })),
    { id: 'open-settings', name: 'Open settings', type: 'system', group: 'System' },
    { id: 'toggle-mode', name: 'Toggle shell mode', type: 'system', group: 'System' },
    { id: 'open-launcher', name: 'Open launcher', type: 'system', group: 'System' },
  ])

  let scoped = $derived(
    scope === 'system'
      ? commands.filter((entry) => entry.type === 'system')
      : commands
  )

  let shown = $derived(
    query.trim()
      ? scoped.filter((entry) => entry.name.toLowerCase().includes(query.toLowerCase())).slice(0, 18)
      : scoped.slice(0, 18)
  )

  $effect(() => {
    if (inputRef) inputRef.focus()
  })

  $effect(() => {
    const q = query.trim()
    if (!q || scope === 'system') {
      backendResults = []
      selected = 0
      return
    }

    const t = setTimeout(async () => {
      try {
        backendResults = await invoke('apps_search', { query: q, limit: 12 })
      } catch (error) {
        console.log('Backend search failed:', error)
      }
    }, 100)

    return () => clearTimeout(t)
  })

  let merged = $derived([
    ...shown,
    ...backendResults.map((app) => ({ id: app.id, name: app.name, type: 'app', group: 'Live' })),
  ].slice(0, 24))

  function keyNav(event) {
    if (event.key === 'ArrowDown') {
      event.preventDefault()
      selected = Math.min(selected + 1, Math.max(merged.length - 1, 0))
    } else if (event.key === 'ArrowUp') {
      event.preventDefault()
      selected = Math.max(selected - 1, 0)
    } else if (event.key === 'Enter') {
      event.preventDefault()
      if (merged[selected]) run(merged[selected])
    } else if (event.key === 'Escape') {
      onclose()
    }
  }

  async function run(item) {
    if (item.type === 'app') {
      try {
        await invoke('apps_launch', { appId: item.id })
      } catch (error) {
        console.log('Run app failed:', error)
      }
    } else {
      onExecute(item.name)
    }
    onclose()
  }

  function initial(name) {
    return (name || '?').slice(0, 1).toUpperCase()
  }
</script>

<div class="overlay" onclick={(event) => event.target === event.currentTarget && onclose()} role="dialog" tabindex="0" onkeydown={keyNav}>
  <div class="palette">
    <header>
      <div>
        <span class="eyebrow">Command Palette</span>
        <h2>Type. Jump. Execute.</h2>
      </div>
      <div class="count">{merged.length}</div>
    </header>

    <div class="search">
      <input type="text" bind:value={query} bind:this={inputRef} placeholder="Search apps or commands" />
      <div class="scope-switch" role="tablist" aria-label="Scope">
        <button class:active={scope === 'all'} onclick={() => (scope = 'all')}>All</button>
        <button class:active={scope === 'system'} onclick={() => (scope = 'system')}>System</button>
      </div>
      <div class="keys">
        <kbd>Esc</kbd>
        <kbd>Enter</kbd>
      </div>
    </div>

    <div class="rows">
      {#if merged.length === 0}
        <div class="empty">No matches. Try a different query.</div>
      {:else}
        {#each merged as item, idx}
          <button class="row" class:active={idx === selected} onclick={() => run(item)} onmouseenter={() => (selected = idx)}>
            <span class="row-main">
              <span class="dot">{initial(item.name)}</span>
              <span>{item.name}</span>
            </span>
            <small>{item.group}</small>
          </button>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    display: flex;
    justify-content: center;
    padding-top: min(12vh, 94px);
    background: linear-gradient(180deg, rgba(5, 9, 20, 0.7), rgba(3, 6, 14, 0.82));
    backdrop-filter: blur(6px);
    z-index: 1900;
  }

  .palette {
    width: min(880px, calc(100vw - 34px));
    max-height: min(72vh, 640px);
    border-radius: 20px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 34%, transparent);
    background:
      radial-gradient(120% 80% at 12% -10%, color-mix(in oklab, var(--accent-a) 16%, transparent), transparent 55%),
      radial-gradient(120% 80% at 92% -20%, color-mix(in oklab, var(--accent-b) 14%, transparent), transparent 58%),
      linear-gradient(180deg, color-mix(in oklab, #141d38 84%, transparent), color-mix(in oklab, #0b1228 86%, transparent));
    box-shadow: 0 30px 76px rgba(2, 9, 22, 0.68), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(18px) saturate(142%);
    color: #eff6ff;
    display: grid;
    grid-template-rows: auto auto 1fr;
    overflow: hidden;
    animation: drop 160ms ease;
  }

  header,
  .search {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 12px 14px;
    border-bottom: 1px solid color-mix(in oklab, var(--accent-a) 16%, transparent);
  }

  .eyebrow {
    font-size: 11px;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: color-mix(in oklab, #eef6ff 56%, transparent);
  }

  h2 {
    margin: 5px 0 0;
    font-size: 24px;
    letter-spacing: -0.02em;
    line-height: 1.04;
  }

  .count,
  input,
  .row,
  .empty,
  .scope-switch button,
  .dot {
    border-radius: 10px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 22%, transparent);
    background: color-mix(in oklab, #edf4ff 8%, transparent);
    color: inherit;
  }

  .count {
    min-width: 36px;
    min-height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 600;
  }

  input {
    flex: 1;
    min-height: 38px;
    padding: 0 12px;
    font-size: 14px;
  }

  .scope-switch {
    display: inline-flex;
    gap: 5px;
    padding: 3px;
    border-radius: 10px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 24%, transparent);
    background: color-mix(in oklab, #edf4ff 4%, transparent);
  }

  .scope-switch button {
    min-height: 26px;
    padding: 0 10px;
    border-radius: 8px;
    border-color: transparent;
    font-size: 12px;
    cursor: pointer;
  }

  .scope-switch button.active {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 24%, transparent), color-mix(in oklab, var(--accent-b) 14%, transparent)),
      color-mix(in oklab, #edf4ff 8%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 42%, transparent);
  }

  .keys {
    display: inline-flex;
    gap: 6px;
  }

  kbd {
    min-width: 24px;
    height: 24px;
    padding: 0 8px;
    border-radius: 8px;
    border: 1px solid color-mix(in oklab, var(--accent-a) 28%, transparent);
    background: color-mix(in oklab, #edf4ff 10%, transparent);
    font-size: 10px;
    color: color-mix(in oklab, #eef6ff 80%, transparent);
    font-family: 'Consolas', 'JetBrains Mono', monospace;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .rows {
    padding: 9px;
    display: grid;
    gap: 6px;
    overflow-y: auto;
  }

  .row {
    min-height: 38px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    font-size: 12px;
    transition: transform 120ms ease, background-color 120ms ease, border-color 120ms ease;
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

  .dot {
    width: 18px;
    height: 18px;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
    font-weight: 600;
    flex-shrink: 0;
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 26%, transparent), color-mix(in oklab, var(--accent-b) 16%, transparent)),
      color-mix(in oklab, #edf4ff 8%, transparent);
  }

  .row small {
    color: color-mix(in oklab, #f2f7ff 55%, transparent);
    font-size: 10px;
  }

  .row.active,
  .row:hover {
    background:
      linear-gradient(120deg, color-mix(in oklab, var(--accent-a) 26%, transparent), color-mix(in oklab, var(--accent-b) 16%, transparent)),
      color-mix(in oklab, #edf4ff 8%, transparent);
    border-color: color-mix(in oklab, var(--accent-a) 50%, transparent);
    transform: translateY(-1px);
  }

  .empty {
    min-height: 42px;
    padding: 0 12px;
    display: inline-flex;
    align-items: center;
    color: color-mix(in oklab, #eff6ff 62%, transparent);
    font-size: 12px;
  }

  @keyframes drop {
    from {
      opacity: 0;
      transform: translateY(-7px) scale(0.99);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
