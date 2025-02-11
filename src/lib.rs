pub mod buffer;
pub mod editor;
pub mod error;
pub mod menu;
pub mod text;
pub mod window;
pub mod ui;

pub struct EeState {
    pub initialized: bool,
}

impl Default for EeState {
    fn default() -> Self {
        Self {
            initialized: false
        }
    }
}