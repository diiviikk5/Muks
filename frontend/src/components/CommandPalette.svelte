<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, onclose, onExecute } = $props()

  let query = $state('')
  let selected = $state(0)
  let backendResults = $state([])
  let inputRef = $state()

  let commands = $derived([
    ...apps.map((app) => ({ id: app.id, name: app.name, type: 'app', group: 'App' })),
    { id: 'open-settings', name: 'Open settings', type: 'system', group: 'System' },
    { id: 'toggle-mode', name: 'Toggle shell mode', type: 'system', group: 'System' },
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

<div class="overlay" onclick={(event) => event.target === event.currentTarget && onclose()} role="dialog" tabindex="0" onkeydown={keyNav}>
  <div class="palette">
    <div class="head">
      <div>
        <span class="kicker">Palette</span>
        <h2>Fast Actions</h2>
      </div>
      <div class="count">{merged.length}</div>
    </div>

    <div class="search">
      <input type="text" bind:value={query} bind:this={inputRef} placeholder="Search app or command" />
      <kbd>Esc</kbd>
    </div>

    <div class="rows">
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
  .overlay {
    position: fixed;
    inset: 0;
    padding-top: 96px;
    display: flex;
    justify-content: center;
    background: rgba(9, 10, 20, 0.6);
    z-index: 1400;
  }

  .palette {
    width: min(740px, calc(100vw - 20px));
    max-height: 510px;
    border-radius: 22px;
    border: 1px solid rgba(173, 194, 255, 0.24);
    background: rgba(29, 33, 56, 0.9);
    backdrop-filter: blur(16px) saturate(126%);
    box-shadow: 0 26px 62px rgba(0, 0, 0, 0.55);
    color: #e9edff;
    display: grid;
    grid-template-rows: auto auto 1fr;
  }

  .head,
  .search {
    padding: 12px 14px;
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
    color: rgba(208, 218, 248, 0.66);
  }

  h2 {
    margin: 3px 0 0;
    font-size: 22px;
  }

  .count,
  input,
  .row {
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.06);
    color: inherit;
  }

  .count {
    min-width: 32px;
    height: 24px;
    display: grid;
    place-items: center;
    font-size: 11px;
  }

  input {
    flex: 1;
    height: 36px;
    padding: 0 10px;
  }

  kbd {
    padding: 5px 8px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.1);
    color: rgba(216, 227, 250, 0.76);
    font-size: 11px;
  }

  .rows {
    padding: 8px;
    display: grid;
    gap: 6px;
    overflow-y: auto;
  }

  .row {
    min-height: 36px;
    padding: 0 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    font-size: 12px;
  }

  .row.active,
  .row:hover {
    background: rgba(127, 200, 255, 0.24);
    border-color: rgba(127, 200, 255, 0.44);
  }

  .row small {
    color: rgba(208, 218, 248, 0.65);
    font-size: 10px;
  }
</style>
