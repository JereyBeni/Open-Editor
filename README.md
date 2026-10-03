# Open Editor

**Lightweight open-source video editor** inspired by VEGAS Pro.

[![Build](https://github.com/JereyBeni/Open-Editor/actions/workflows/build.yml/badge.svg)](https://github.com/JereyBeni/Open-Editor/actions/workflows/build.yml)

- UI written in **Rust** (egui)
- Support for **DirectFX** (legacy Vegas effects)
- Full **OpenFX** host (BCC + SapphireFX as priority)
- Extremely low resource usage (targets 1GB RAM machines)
- Windows + Linux (Android planned)

## Download

Go to the [Actions](https://github.com/JereyBeni/Open-Editor/actions) tab → latest successful workflow → download:

- **Open-Editor-Windows** → `open-editor.exe`
- **Open-Editor-Linux** → binary

Or wait for the official Releases (coming soon).

## Features (planned / in progress)

### Effects Priority
1. **DirectFX** – old Vegas plugins
2. **OpenFX**:
   - Boris Continuum Complete (BCC)
   - SapphireFX
   - Free plugins later (openfx-misc, G'MIC, etc.)

### Editor
- Timeline
- Media bin
- Preview
- Effects panel (Vegas style)
- Low memory footprint

## Building from source

```bash
git clone https://github.com/JereyBeni/Open-Editor.git
cd Open-Editor
cargo build --release
```

### Requirements
- Rust (stable)
- Linux: `libgtk-3-dev` and related packages

## Related repo

Lightweight OFX effects we are developing:
→ [OpenFX-Vegas-Lite](https://github.com/JereyBeni/OpenFX-Vegas-Lite)

## License

MIT

---

Made for people who still rock old PCs and love VEGAS 🔥