# VORTEX Shell

A living, intelligent desktop shell for Windows - built with Rust.

## Features

- **Lightweight**: Target <80MB idle RAM (vs 300-600MB competitors)
- **Biome System**: 5 unique visual themes with smooth transitions
  - AuraFlow - Fluid gradient animations
  - CrystalEdge - Sharp, refractive aesthetics
  - MidnightPulse - Dark mode with ambient glow
  - ForestHaze - Nature-inspired particles
  - ZenStone - Minimal ripple effects
- **AI-Ready**: Local-first intelligent agents (ONNX integration)
- **Modular**: Built with Rust for performance and safety

## Tech Stack

- **Language**: Rust
- **Platform**: Windows API (windows-sys)
- **Build**: Cargo

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Running

```bash
# Run debug
cargo run

# Run release
cargo run --release
```

## Architecture

```
src/
├── main.rs         # Entry point
├── core/           # Core shell state and events
├── platform/       # Windows API integration
├── shell/          # Taskbar and window management
├── biomes/         # Visual theme system
├── ai/             # AI agent framework
└── rendering/      # GPU renderer (placeholder)
```

## Roadmap

- [x] Core architecture
- [x] Biome system
- [x] AI framework
- [ ] Real window creation (Win32)
- [ ] GPU-accelerated rendering
- [ ] System tray integration
- [ ] ONNX model integration

## Comparison

| Feature | VORTEX | Seelen UI | Cairo Desktop |
|---------|--------|-----------|---------------|
| Idle RAM (target) | <80MB | 300-600MB | 150-300MB |
| AI Agents | Local-first | None | None |
| Biome System | Dynamic | Static | None |
| Customization | Theme engine | Heavy | Minimal |

## License

MIT
