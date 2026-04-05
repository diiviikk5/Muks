<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, onclose, onExecute } = $props()

  let query = $state('')
  let selected = $state(0)
  let backendResults = $state([])
  let inputRef = $state()

  let commands = $derived([
    ...apps.map((app) => ({ id: app.id, name: app.name, type: 'app', group: 'App' })),
    { id: 'restart-shell', name: 'Restart shell (placeholder)', type: 'system', group: 'System' },
    { id: 'open-settings', name: 'Open settings', type: 'system', group: 'System' },
  ])

  let shown = $derived(
    query.trim()
      ? commands.filter((entry) => entry.name.toLowerCase().includes(query.toLowerCase())).slice(0, 16)
      : commands.slice(0, 16)
  )

  $effect(() => {
    if (inputRef) inputRef.focus()
  })

  $effect(() => {
    const q = query.trim()
    if (!q) {
      backendResults = []
      return
    }

    const t = setTimeout(async () => {
      try {
        backendResults = await invoke('apps_search', { query: q, limit: 12 })
      } catch (error) {
        console.log('Backend search failed:', error)
      }
    }, 120)

    return () => clearTimeout(t)
  })

  let merged = $derived([
    ...shown,
    ...backendResults.map((app) => ({ id: app.id, name: app.name, type: 'app', group: 'Live' })),
  ].slice(0, 20))

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
</script>

<div class="palette-overlay" onclick={(event) => event.target === event.currentTarget && onclose()} role="dialog" tabindex="0" onkeydown={keyNav}>
  <div class="palette">
    <div class="top">
      <div>
        <span class="kicker">Command Palette</span>
        <h2>Launch fast</h2>
      </div>
      <div class="badge">{merged.length}</div>
    </div>

    <div class="search">
      <input type="text" bind:value={query} bind:this={inputRef} placeholder="Search apps and shell commands" />
      <kbd>Esc</kbd>
    </div>

    <div class="results">
      {#each merged as item, idx}
        <button class="row" class:active={idx === selected} onclick={() => run(item)} onmouseenter={() => (selected = idx)}>
          <span>{item.name}</span>
          <small>{item.group}</small>
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .palette-overlay {
    position: fixed;
    inset: 0;
    padding-top: 120px;
    display: flex;
    justify-content: center;
    background: rgba(5, 8, 14, 0.56);
    z-index: 1400;
  }

  .palette {
    width: min(760px, calc(100vw - 20px));
    max-height: 520px;
    border-radius: 24px;
    border: 1px solid rgba(163, 190, 255, 0.22);
    background: rgba(10, 16, 26, 0.9);
    backdrop-filter: blur(18px) saturate(126%);
    box-shadow: 0 28px 70px rgba(0, 0, 0, 0.55);
    color: #eaf2ff;
    display: grid;
    grid-template-rows: auto auto 1fr;
  }

  .top,
  .search {
    padding: 14px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }

  .kicker {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(214, 227, 255, 0.62);
  }

  h2 {
    margin: 3px 0 0;
    font-size: 24px;
    line-height: 1;
  }

  .badge {
    min-width: 34px;
    height: 28px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.06);
    display: grid;
    place-items: center;
    font-size: 12px;
  }

  .search input {
    flex: 1;
    height: 40px;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.05);
    color: #f3f8ff;
    padding: 0 12px;
  }

  .search kbd {
    padding: 6px 9px;
    border-radius: 9px;
    background: rgba(255, 255, 255, 0.08);
    color: rgba(215, 228, 255, 0.74);
    font-size: 11px;
  }

  .results {
    padding: 10px;
    display: grid;
    gap: 6px;
    overflow-y: auto;
  }

  .row {
    min-height: 40px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    text-align: left;
    border-radius: 11px;
    border: 1px solid rgba(255, 255, 255, 0.07);
    background: rgba(255, 255, 255, 0.04);
    color: inherit;
    cursor: pointer;
    font-size: 13px;
  }

  .row.active,
  .row:hover {
    background: rgba(102, 167, 255, 0.24);
    border-color: rgba(102, 167, 255, 0.45);
  }

  .row small {
    font-size: 11px;
    color: rgba(214, 227, 255, 0.62);
  }
</style>
