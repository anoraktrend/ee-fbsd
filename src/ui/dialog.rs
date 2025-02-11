use std::path::PathBuf;

#[derive(Debug)]
pub enum DialogResult {
    None,
    Cancel,
    Save(PathBuf),
    Load(PathBuf),
}

#[derive(Debug, Clone)]  // Add Clone derivation
pub enum DialogType {
    Save,
    Load,
}

impl DialogType {
    pub fn title(&self) -> &str {
        match self {
            DialogType::Save => "Save As",
            DialogType::Load => "Open File",
        }
    }
}
