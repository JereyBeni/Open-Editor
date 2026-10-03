// Open Editor - Plugin system
// Priority:
// 1. DirectFX (legacy Vegas effects)
// 2. OFX with focus on BCC and SapphireFX compatibility

pub mod directfx;
pub mod ofx;

/// Common trait for all effects (DirectFX or OFX)
pub trait Effect {
    fn name(&self) -> &str;
    fn render(&mut self /* frame data later */);
}

/// Plugin manager
pub struct PluginManager {
    // Will hold loaded DirectFX and OFX plugins
}

impl PluginManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan_directfx(&mut self) {
        // TODO: scan for .dll DirectFX plugins (old Vegas format)
    }

    pub fn scan_ofx(&mut self) {
        // TODO: scan standard OFX folders
        // Priority test hosts: BCC and SapphireFX
    }
}