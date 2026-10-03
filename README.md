# Open Editor

**Lightweight open-source video editor** inspired by VEGAS Pro.

- **UI written in Rust** (egui / iced / custom)
- Recreates classic VEGAS Pro effects
- Full **OpenFX (OFX)** plugin support
- Designed for low-spec machines:
  - 1 GB RAM minimum
  - Windows XP → Windows 11
  - DDR3 era hardware friendly

## Goals
- Timeline-based editor
- Native OFX host (so you can load Gradient, Channel Blend, Glow, etc.)
- Extremely low memory footprint
- Fast on old CPUs
- No Electron, no heavy frameworks

## Tech Stack (planned)
- **UI**: Rust (egui preferred for simplicity and low overhead)
- **Video engine**: FFmpeg (static) or custom lightweight decoder
- **Effects**: Built-in + OpenFX host
- **Rendering**: Software first, optional GPU later

## Related
Effects plugins live in a separate repo for modularity:
→ [OpenFX-Vegas-Lite](https://github.com/JereyBeni/OpenFX-Vegas-Lite)

## Status
Very early stage. UI skeleton coming next.

---
Made for people who still rock old PCs 🔥