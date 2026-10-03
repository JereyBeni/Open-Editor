// OpenFX host
// First priority plugins to support well:
// 1. Boris Continuum Complete (BCC)
// 2. SapphireFX (Boris FX Sapphire)

pub struct OfxPlugin {
    pub name: String,
    pub vendor: String,
    // ofx handle, bundle path, etc.
}

impl OfxPlugin {
    pub fn load(_bundle_path: &str) -> Option<Self> {
        // TODO: Use libloading + OpenFX C API
        // Focus first on making BCC and Sapphire load without crashes
        None
    }
}

/// Standard OFX search paths (Windows)
pub fn standard_ofx_paths() -> Vec<String> {
    vec![
        r"C:\Program Files\Common Files\OFX\Plugins".to_string(),
        r"C:\Program Files (x86)\Common Files\OFX\Plugins".to_string(),
        // BCC and Sapphire usually install here
    ]
}

/// Priority vendors we want to support first
pub const PRIORITY_VENDORS: &[&str] = &[
    "BorisFX",           // BCC + Sapphire
    "Boris Continuum",   
    "Sapphire",
];