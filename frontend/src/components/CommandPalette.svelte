<script>
  import { invoke } from '@tauri-apps/api/core'

  let { apps, onclose, onExecute } = $props()

  let query = $state('')
  let selectedIndex = $state(0)
  let inputRef = $state()
  let backendResults = $state([])

  const commandGroups = [
    { id: 'explorer', name: 'Open File Explorer', type: 'app', icon: 'folder', group: 'Launch' },
    { id: 'browser', name: 'Open Browser', type: 'app', icon: 'globe', group: 'Launch' },
    { id: 'terminal', name: 'Open Terminal', type: 'app', icon: 'terminal', group: 'Launch' },
    { id: 'settings', name: 'Open Settings', type: 'app', icon: 'settings', group: 'Launch' },
    { id: 'shutdown', name: 'Shutdown', type: 'system', icon: 'power', group: 'System' },
    { id: 'restart', name: 'Restart', type: 'system', icon: 'power', group: 'System' },
    { id: 'sleep', name: 'Sleep', type: 'system', icon: 'power', group: 'System' },
  ]

  let commands = $derived([
    ...commandGroups,
    ...apps.slice(0, 40).map((app) => ({
      id: app.id,
      name: app.name,
      type: 'app',
      icon: 'folder',
      group: 'Indexed',
    })),
    ...backendResults.map((app) => ({
      id: app.id,
      name: app.name,
      type: 'app',
      icon: 'folder',
      group: 'Live Search',
    })),
  ])

  let filteredCommands = $derived(
    query.trim()
      ? commands.filter((command) => command.name.toLowerCase().includes(query.toLowerCase()))
      : commands
  )

  $effect(() => {
    if (inputRef) {
      inputRef.focus()
    }
  })

  $effect(() => {
    const activeQuery = query.trim()

    if (!activeQuery) {
      backendResults = []
      return
    }

    const timer = setTimeout(async () => {
      try {
        backendResults = await invoke('apps_search', { query: activeQuery, limit: 20 })
      } catch (error) {
        console.log('Backend search error:', error)
      }
    }, 120)

    return () => clearTimeout(timer)
  })

  function handleKeydown(event) {
    if (event.key === 'ArrowDown') {
      event.preventDefault()
      selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1)
    } else if (event.key === 'ArrowUp') {
      event.preventDefault()
      selectedIndex = Math.max(selectedIndex - 1, 0)
    } else if (event.key === 'Enter') {
      event.preventDefault()
      if (filteredCommands[selectedIndex]) {
        executeCommand(filteredCommands[selectedIndex])
      }
    }
  }

  async function executeCommand(command) {
    if (command.type === 'app') {
      try {
        await invoke('apps_launch', { appId: command.id })
      } catch (error) {
        console.log('Execute error:', error)
      }
    } else {
      onExecute(command.name)
    }
    onclose()
  }

  function getIcon(icon) {
    switch (icon) {
      case 'folder':
        return 'M10 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z'
      case 'globe':
        return 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z'
      case 'terminal':
        return 'M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V8h16v10z'
      case 'settings':
        return 'M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58a.49.49 0 0 0 .12-.61l-1.92-3.32a.488.488 0 0 0-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54a.484.484 0 0 0-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.04.31-.06.63-.06.94s.02.63.06.94l-2.03 1.58a.49.49 0 0 0-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58z'
      case 'power':
        return 'M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z'
      default:
        return ''
    }
  }
</script>

<div
  class="palette-overlay"
  onclick={(event) => event.target === event.currentTarget && onclose()}
  onkeydown={handleKeydown}
  role="dialog"
  tabindex="0"
>
  <div class="palette" role="document">
    <div class="palette-topbar">
      <div>
        <span class="eyebrow">VORTEX Search</span>
        <h2>Command Surface</h2>
      </div>
      <div class="command-badge">{filteredCommands.length} results</div>
    </div>

    <div class="search-container">
      <div class="search-icon">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
      </div>
      <input
        type="text"
        placeholder="Search apps, settings, and shell actions..."
        bind:value={query}
        bind:this={inputRef}
      />
      <kbd>ESC</kbd>
    </div>

    <div class="results">
      {#each filteredCommands as command, index}
        <button
          class="result-item"
          class:selected={index === selectedIndex}
          onclick={() => executeCommand(command)}
          onmouseenter={() => (selectedIndex = index)}
        >
          <div class="result-icon" class:system={command.type === 'system'}>
            <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
              <path d={getIcon(command.icon)} />
            </svg>
          </div>
          <div class="result-copy">
            <strong>{command.name}</strong>
            <span>{command.group}</span>
          </div>
          <span class="result-type">{command.type}</span>
        </button>
      {/each}
    </div>

    <div class="footer">
      <div class="footer-hint"><kbd>Up/Down</kbd> Navigate</div>
      <div class="footer-hint"><kbd>Enter</kbd> Launch</div>
      <div class="footer-hint"><kbd>Alt+Space</kbd> Toggle</div>
    </div>
  </div>
</div>

<style>
  .palette-overlay {
    position: fixed;
    inset: 0;
    display: flex;
    justify-content: center;
    padding-top: 132px;
    background:
      linear-gradient(180deg, rgba(5, 7, 10, 0.22), rgba(5, 7, 10, 0.56));
    z-index: 1000;
    animation: fadeIn 0.12s ease;
  }

  .palette {
    width: 720px;
    max-height: 520px;
    display: grid;
    grid-template-rows: auto auto 1fr auto;
    border-radius: 30px;
    overflow: hidden;
    color: #eef4ff;
    background:
      linear-gradient(180deg, rgba(22, 29, 41, 0.98), rgba(9, 13, 19, 0.98)),
      radial-gradient(circle at top right, rgba(91, 155, 255, 0.22), transparent 40%);
    border: 1px solid rgba(255, 255, 255, 0.09);
    backdrop-filter: blur(36px) saturate(140%);
    box-shadow:
      0 30px 90px rgba(0, 0, 0, 0.55),
      inset 0 1px 0 rgba(255, 255, 255, 0.06);
  }

  .palette-topbar {
    display: flex;
    align-items: end;
    justify-content: space-between;
    padding: 22px 24px 14px;
  }

  .eyebrow,
  .result-copy span,
  .command-badge,
  .result-type {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(214, 227, 255, 0.58);
  }

  .palette-topbar h2 {
    margin: 4px 0 0;
    font-size: 28px;
    line-height: 1;
  }

  .command-badge {
    padding: 10px 14px;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.06);
  }

  .search-container {
    display: flex;
    align-items: center;
    gap: 12px;
    margin: 0 24px 16px;
    padding: 14px 16px;
    border-radius: 22px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.05);
  }

  .search-icon {
    color: rgba(214, 227, 255, 0.62);
  }

  .search-container input {
    flex: 1;
    background: none;
    border: none;
    outline: none;
    color: white;
    font-size: 16px;
  }

  .search-container kbd,
  .footer-hint kbd {
    padding: 4px 8px;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.08);
    color: rgba(214, 227, 255, 0.68);
  }

  .results {
    padding: 0 16px 12px;
    overflow-y: auto;
  }

  .result-item {
    width: 100%;
    display: grid;
    grid-template-columns: 42px 1fr auto;
    align-items: center;
    gap: 14px;
    padding: 12px 14px;
    border: none;
    border-radius: 20px;
    background: transparent;
    color: inherit;
    cursor: pointer;
    text-align: left;
    transition: transform 0.14s ease, background 0.14s ease;
  }

  .result-item:hover,
  .result-item.selected {
    background: rgba(255, 255, 255, 0.075);
    transform: translateY(-1px);
  }

  .result-icon {
    width: 42px;
    height: 42px;
    display: grid;
    place-items: center;
    border-radius: 16px;
    background:
      linear-gradient(135deg, rgba(255, 255, 255, 0.1), rgba(255, 255, 255, 0.02)),
      rgba(91, 155, 255, 0.14);
    color: #a9c9ff;
  }

  .result-icon.system {
    background:
      linear-gradient(135deg, rgba(255, 220, 127, 0.18), rgba(255, 255, 255, 0.02)),
      rgba(255, 195, 74, 0.12);
    color: #ffd387;
  }

  .result-copy {
    display: grid;
    gap: 4px;
  }

  .result-copy strong {
    font-size: 14px;
    font-weight: 700;
  }

  .footer {
    display: flex;
    gap: 16px;
    padding: 14px 24px 18px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
    background: rgba(255, 255, 255, 0.02);
  }

  .footer-hint {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: rgba(214, 227, 255, 0.56);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
