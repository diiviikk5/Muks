<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, onclose, onExecute } = $props()

  let query = $state('')
  let selected = $state(0)
  let backendResults = $state([])
  let inputRef = $state()

  let commands = $derived([
    ...apps.map((app) => ({ id: app.id, name: app.name, type: 'app', group: 'Apps' })),
    { id: 'open-settings', name: 'Open settings', type: 'system', group: 'System' },
    { id: 'toggle-mode', name: 'Toggle shell mode', type: 'system', group: 'System' },
    { id: 'open-launcher', name: 'Open launcher', type: 'system', group: 'System' },
  ])

  let shown = $derived(
    query.trim()
      ? commands.filter((entry) => entry.name.toLowerCase().includes(query.toLowerCase())).slice(0, 18)
      : commands.slice(0, 18)
  )

  $effect(() => {
    if (inputRef) inputRef.focus()
  })

  $effect(() => {
    const q = query.trim()
    if (!q) {
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
    }, 110)

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
      <input type="text" bind:value={query} bind:this={inputRef} placeholder="Search apps, commands, and automations" />
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
              <span class="dot"></span>
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
    padding-top: min(14vh, 120px);
    background: linear-gradient(180deg, rgba(5, 9, 20, 0.74), rgba(3, 6, 14, 0.84));
    backdrop-filter: blur(6px);
    z-index: 1900;
  }

  .palette {
    width: min(860px, calc(100vw - 34px));
    max-height: min(70vh, 620px);
    border-radius: 24px;
    border: 1px solid color-mix(in oklab, #99bcff 36%, transparent);
    background:
      radial-gradient(120% 80% at 12% -10%, color-mix(in oklab, #89c7ff 16%, transparent), transparent 55%),
      radial-gradient(120% 80% at 92% -20%, color-mix(in oklab, #b390ff 14%, transparent), transparent 58%),
      linear-gradient(180deg, color-mix(in oklab, #141d38 84%, transparent), color-mix(in oklab, #0b1228 86%, transparent));
    box-shadow: 0 34px 80px rgba(2, 9, 22, 0.7), inset 0 1px 0 rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(20px) saturate(145%);
    color: #eff6ff;
    display: grid;
    grid-template-rows: auto auto 1fr;
    overflow: hidden;
    animation: drop 170ms ease;
  }

  header,
  .search {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 14px 16px;
    border-bottom: 1px solid color-mix(in oklab, #bdd2ff 16%, transparent);
  }

  .eyebrow {
    font-size: 11px;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    color: color-mix(in oklab, #eef6ff 56%, transparent);
  }

  h2 {
    margin: 6px 0 0;
    font-size: 29px;
    letter-spacing: -0.03em;
    line-height: 1;
  }

  .count,
  input,
  .row,
  .empty {
    border-radius: 12px;
    border: 1px solid color-mix(in oklab, #c2d6ff 23%, transparent);
    background: color-mix(in oklab, #edf4ff 8%, transparent);
    color: inherit;
  }

  .count {
    min-width: 40px;
    min-height: 30px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 600;
  }

  input {
    flex: 1;
    min-height: 42px;
    padding: 0 14px;
    font-size: 15px;
  }

  .keys {
    display: inline-flex;
    gap: 6px;
  }

  kbd {
    min-width: 28px;
    height: 26px;
    padding: 0 8px;
    border-radius: 8px;
    border: 1px solid color-mix(in oklab, #c5d9ff 28%, transparent);
    background: color-mix(in oklab, #edf4ff 10%, transparent);
    font-size: 11px;
    color: color-mix(in oklab, #eef6ff 80%, transparent);
    font-family: 'Consolas', 'JetBrains Mono', monospace;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .rows {
    padding: 10px;
    display: grid;
    gap: 7px;
    overflow-y: auto;
  }

  .row {
    min-height: 42px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    font-size: 13px;
    transition: transform 140ms ease, background-color 140ms ease, border-color 140ms ease;
  }

  .row-main {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 100px;
    background: color-mix(in oklab, #8fc9ff 92%, white 8%);
    box-shadow: 0 0 12px color-mix(in oklab, #8fc9ff 56%, transparent);
    flex-shrink: 0;
  }

  .row small {
    color: color-mix(in oklab, #f2f7ff 55%, transparent);
    font-size: 11px;
  }

  .row.active,
  .row:hover {
    background:
      linear-gradient(120deg, color-mix(in oklab, #89c4ff 26%, transparent), color-mix(in oklab, #bb94ff 16%, transparent)),
      color-mix(in oklab, #edf4ff 8%, transparent);
    border-color: color-mix(in oklab, #9ec2ff 50%, transparent);
    transform: translateY(-1px);
  }

  .empty {
    min-height: 44px;
    padding: 0 12px;
    display: inline-flex;
    align-items: center;
    color: color-mix(in oklab, #eff6ff 62%, transparent);
    font-size: 13px;
  }

  @keyframes drop {
    from {
      opacity: 0;
      transform: translateY(-8px) scale(0.985);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
