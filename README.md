# Open Editor

**Lightweight open-source video editor** inspired by VEGAS Pro.

- **UI written in Rust** (egui)
- Recreates classic VEGAS Pro effects
- **DirectFX** effects support (legacy Vegas plugins)
- Full **OpenFX (OFX)** host
- Designed for low-spec machines (1GB+ RAM, Windows XP → 11)

## First Priorities

### 1. DirectFX Support
Legacy Vegas DirectFX / DirectX Media style effects (the old plugin system).

### 2. OFX Plugins (priority order)
1. **Boris Continuum Complete (BCC)**
2. **SapphireFX** (Boris FX Sapphire)
3. Then free/open plugins (openfx-misc, G'MIC, etc.)

> Note: BCC and Sapphire are commercial. Open Editor will be a proper OFX host so you can load your own licensed copies.

## Goals
- Timeline-based editor
- Native OFX + DirectFX host
- Extremely low memory footprint
- Fast on old CPUs
- No Electron, no heavy frameworks

## Tech Stack (planned)
- **UI**: Rust + egui
- **Video engine**: FFmpeg (static) or lightweight custom
- **Effects**: Built-in + DirectFX + OpenFX host
- **Rendering**: Software first

## Related
Lightweight OFX effects we develop:
→ [OpenFX-Vegas-Lite](https://github.com/JereyBeni/OpenFX-Vegas-Lite)

## Status
Early stage. Next up: DirectFX loading + OFX host skeleton focused on BCC/Sapphire compatibility.

---
Made for people who still rock old PCs 🔥