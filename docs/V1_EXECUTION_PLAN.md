# VORTEX Shell V1 Execution Plan

## Product Pillars
- Performance-first shell replacement with low idle memory and smooth interactions.
- High customization for normal users and power users.
- Polished UX with safe defaults and deep advanced controls.

## Milestone 0: Stabilize Foundation (current sprint)
- Keep Rust + Tauri + Svelte stack as the single source of truth.
- Harden startup/shutdown behavior so Windows taskbar is restored on all exits.
- Remove template leftovers and broken command wiring.
- Add baseline perf telemetry (startup time, FPS, memory snapshots).

## Milestone 1: Core Shell Runtime
- Implement reliable app indexer with cached catalog and incremental refresh.
- Add robust launcher/search scoring and keyboard navigation.
- Add shell session service for global state (workspaces, focused window, pinned apps).
- Define event bus contracts between Rust backend and Svelte frontend.

## Milestone 2: Window Management
- Introduce Win32 event hooks and per-window metadata tracking.
- Support core actions: focus, minimize, maximize, close, move-to-workspace.
- Add rules engine (per-app behavior, startup workspace, float/tile policy).
- Add safe fallback mode if window hooks fail.

## Milestone 3: Biomes + Theming System
- Build biome renderer contract (palette, blur, animation, background effects).
- Add live theme editor and hot reload from TOML/JSON.
- Support import/export of themes and visual presets.
- Ensure all visuals degrade cleanly on low-end hardware.

## Milestone 4: Plugin + Widget Platform
- Define plugin manifest, permission model, and lifecycle.
- Enable widget surface with sandboxed APIs.
- Add plugin store primitives (install, update, disable, version pinning).
- Add crash isolation and watchdogs for third-party extensions.

## Milestone 5: AI Assist Layer (optional-by-default)
- Keep AI fully off unless user explicitly enables it.
- Add provider abstraction (local model first, cloud optional).
- Add shell-safe use-cases: command help, rule suggestions, theme generation.
- Add strict resource and privacy controls.

## Milestone 6: Packaging + Reliability
- Implement installer and updater flow with clean rollback.
- Add first-run onboarding with presets (Windows/macOS/Linux styles).
- Add diagnostics bundle and bug-report shortcuts.
- Complete accessibility pass (keyboard-only, high-contrast, readable scaling).

## Engineering Standards
- Every milestone ships with benchmarks and automated regression checks.
- No feature merges without fallback behavior and startup safety guarantees.
- Keep memory budget and startup targets visible in CI.

## Immediate Next Sprint (recommended)
1. Add backend event bus and frontend state store contracts.
2. Implement persistent app catalog cache with invalidation.
3. Build workspace model in Rust and expose Tauri commands/events.
4. Add integration tests for startup, shutdown, and shell recovery paths.
