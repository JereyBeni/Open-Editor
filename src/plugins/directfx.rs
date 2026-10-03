// DirectFX support
// Old Vegas Pro plugin format (DirectX Media / Vegas native plugins)
// Priority #1 for compatibility with classic effects

pub struct DirectFxPlugin {
    pub name: String,
    // path, handle, etc later
}

impl DirectFxPlugin {
    pub fn load(_path: &str) -> Option<Self> {
        // TODO: Load .dll using LoadLibrary + GetProcAddress
        // Vegas DirectFX uses specific entry points
        None
    }
}

/// Scan common Vegas DirectFX folders
pub fn scan_directfx_folders() -> Vec<String> {
    let mut paths = Vec::new();

    // Classic locations
    paths.push(r"C:\Program Files\Sony\Vegas Pro\DirectFX".to_string());
    paths.push(r"C:\Program Files\VEGAS\VEGAS Pro\DirectFX".to_string());
    paths.push(r"C:\Program Files (x86)\Sony\Vegas Pro\DirectFX".to_string());

    // User can add more later
    paths
}