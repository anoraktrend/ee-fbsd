use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub struct Settings {
    mouse_enabled: Arc<AtomicBool>,
    dos_mode: Arc<AtomicBool>,
    auto_indent: Arc<AtomicBool>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            mouse_enabled: Arc::new(AtomicBool::new(false)),
            dos_mode: Arc::new(AtomicBool::new(false)),
            auto_indent: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn toggle_mouse(&self) -> bool {
        let new_value = !self.mouse_enabled.load(Ordering::Relaxed);
        self.mouse_enabled.store(new_value, Ordering::Relaxed);
        new_value
    }

    pub fn toggle_dos_mode(&self) -> bool {
        let new_value = !self.dos_mode.load(Ordering::Relaxed);
        self.dos_mode.store(new_value, Ordering::Relaxed);
        new_value
    }

    pub fn toggle_auto_indent(&self) -> bool {
        let new_value = !self.auto_indent.load(Ordering::Relaxed);
        self.auto_indent.store(new_value, Ordering::Relaxed);
        new_value
    }

    pub fn is_mouse_enabled(&self) -> bool {
        self.mouse_enabled.load(Ordering::Relaxed)
    }

    pub fn is_dos_mode(&self) -> bool {
        self.dos_mode.load(Ordering::Relaxed)
    }

    pub fn is_auto_indent(&self) -> bool {
        self.auto_indent.load(Ordering::Relaxed)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
