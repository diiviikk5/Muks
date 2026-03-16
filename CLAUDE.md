# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

VORTEX Shell is a lightweight, intelligent desktop shell for Windows built with Rust + Tauri v2. It aims to replace the Windows taskbar with a modern alternative featuring dynamic visual themes ("biomes") and local-first AI capabilities.

**Target**: <80MB idle RAM (vs 300-600MB competitors)
**Binary size**: ~7.6MB (release build)

## Common Commands

```bash
# Build debug version
cargo build

# Build release version (optimized, LTO enabled)
cargo build --release

# Run debug version
cargo run

# Run release version
cargo run --release
```

## Architecture

The project uses **Tauri v2** for the GUI layer with a Rust backend:

### Frontend (`dist/`)
- **index.html** - Taskbar UI with HTML/CSS, transparent background, clock, start button, system tray

### Backend (`src/`)
- **main.rs** - Binary entry point
- **lib.rs** - Tauri application setup, commands, window positioning
- **core/** - Core shell state (`VortexCore`), event types
- **shell/** - Shell manager for window management
- **biomes/** - Visual theme system (5 biome types)
- **ai/** - AI agent framework (placeholder for ONNX)
- **config/** - TOML configuration
- **platform/** - Windows API integration (still being ported)

### Configuration
- **tauri.conf.json** - Tauri configuration (transparent window, frameless, always-on-top, skip taskbar)
- **capabilities/default.json** - Tauri permissions

## Key Implementation Details

- **Tauri v2**: Uses WebView2 for UI rendering
- **Transparent window**: Configured in tauri.conf.json with `transparent: true`
- **Frameless window**: `decorations: false` for custom taskbar
- **Always on top**: `alwaysOnTop: true` to stay above desktop
- **Skip taskbar**: `skipTaskbar: true` for shell replacement
- **Logging**: Uses `tracing` with `tracing-subscriber`

## Roadmap

- [x] Core architecture
- [x] Tauri v2 integration
- [x] Basic taskbar UI
- [ ] Biome system rendering in UI
- [ ] Window management (minimize, maximize, close)
- [ ] Start menu
- [ ] System tray integration
- [ ] AI features
