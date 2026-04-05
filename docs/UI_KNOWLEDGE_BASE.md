# UI Knowledge Base (Locked Input for UI Phase)

This document captures product direction for "best UI/UX ever" without changing the existing implementation plan.

## Core Direction
- Keep frontend on Tauri 2 + Svelte/Vite + WebView2 for maximum customization and lightweight runtime.
- Prioritize user-editable styling via CSS variables, with optional advanced SCSS layering.
- Use OKLCH-capable token system for dynamic light/dark/auto theming.

## Rendering and Effects
- Use Windows visual backdrops (Mica/Acrylic/Desktop Acrylic) from Rust side where supported.
- Keep custom-frameless window shell with drag regions and polished controls.
- Reserve heavy visuals (shader effects, particles, reactive layers) to GPU-backed surfaces and gate by perf tier.
- Ensure all effects have graceful fallback paths on low-end hardware.

## Widget Engine Principles
- Widgets are isolated, self-contained packages (UI + config schema + metadata).
- Support drag/drop placement, snap grid, and free positioning.
- Provide visual config editor for non-technical users; raw config/code path for power users.
- Allow global theme inheritance with per-widget override tokens.

## Theme System Principles
- Theme artifacts are importable bundles with metadata + palette + overrides + optional assets.
- Theme application is hot-reload, restart-free.
- Support layering: global shell theme, panel theme, widget theme.
- Add wallpaper-reactive palette extraction and optional AI-assisted palette generation later.

## UX Quality Bar
- Preset bundles (e.g., Fluent, Minimal Glass, Cyberpunk, etc.) should apply full layouts/themes quickly.
- Accessibility toggles must remain compatible with custom themes (high contrast, reduced motion, keyboard-first).
- Include perf visibility for users (fps/widget cost) to keep customization safe.
- Maintain screenshot-worthy visual polish while preserving startup and interaction performance budgets.

## Stack Notes (UI)
- Styling stack preference: Tailwind + CSS variables + optional SCSS support.
- Effects stack preference: window-vibrancy + WebGPU/WebGL layers where justified.
- Extensibility preference: widget packages + plugin extensions with secure boundaries.
