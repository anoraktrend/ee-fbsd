use crate::config::EditorConfig;

pub struct Menu {
    title: String,
    items: Vec<MenuItem>,
    selected: usize,
    width: u16,
}

pub struct MenuItem {
    text: String,
    action: MenuAction,
}

pub enum MenuAction {
    None,
    Close,
    Save,
    SaveAs,
    Exit,
    Search,
    NewFile,
    CloseTab,
    NextTab,
    PrevTab,
    ToggleSetting(String),
    KeyBinding(String),
    SaveChanges,
    DiscardChanges,
    Cancel,
    FindNext,
    Replace,
    Custom(Box<dyn Fn() -> bool>),
    KeyExit,
    KeyWrite,
    KeySearch,
    KeyMenu,
    KeyNext,
    KeyPrev,
    KeyCut,
    KeyPaste,
    SaveConfig,
}

pub struct InputDialog {
    prompt: String,
    width: u16,
    input: String,
}

impl Menu {
    pub fn new(title: &str) -> Self {
        Menu {
            title: title.to_string(),
            items: Vec::new(),
            selected: 0,
            width: 40,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn add_item(&mut self, text: &str, action: MenuAction) {
        self.items.push(MenuItem {
            text: text.to_string(),
            action,
        });
    }

    pub fn handle_input(&mut self, key: char) -> Option<MenuAction> {
        match key {
            '\n' => Some(self.items.get(self.selected)?.action.clone()),
            'j' | 'B' => {  // Down arrow
                self.selected = (self.selected + 1) % self.items.len();
                None
            },
            'k' | 'A' => {  // Up arrow
                self.selected = self.selected.checked_sub(1).unwrap_or(self.items.len() - 1);
                None
            },
            _ => None,
        }
    }
    
    pub fn show(&mut self) -> Option<MenuAction> {
        // Display menu and handle input until action selected
        None // Temporary implementation
    }
}

impl Clone for MenuAction {
    fn clone(&self) -> Self {
        match self {
            MenuAction::None => MenuAction::None,
            MenuAction::Close => MenuAction::Close,
            MenuAction::Save => MenuAction::Save,
            MenuAction::SaveAs => MenuAction::SaveAs,
            MenuAction::Exit => MenuAction::Exit,
            MenuAction::Search => MenuAction::Search,
            MenuAction::NewFile => MenuAction::NewFile,
            MenuAction::CloseTab => MenuAction::CloseTab,
            MenuAction::NextTab => MenuAction::NextTab,
            MenuAction::PrevTab => MenuAction::PrevTab,
            MenuAction::ToggleSetting(s) => MenuAction::ToggleSetting(s.clone()),
            MenuAction::KeyBinding(s) => MenuAction::KeyBinding(s.clone()),
            MenuAction::SaveChanges => MenuAction::SaveChanges,
            MenuAction::DiscardChanges => MenuAction::DiscardChanges,
            MenuAction::Cancel => MenuAction::Cancel,
            MenuAction::FindNext => MenuAction::FindNext,
            MenuAction::Replace => MenuAction::Replace,
            MenuAction::Custom(_) => MenuAction::None, // Custom actions can't be cloned
            MenuAction::KeyExit => MenuAction::KeyExit,
            MenuAction::KeyWrite => MenuAction::KeyWrite,
            MenuAction::KeySearch => MenuAction::KeySearch,
            MenuAction::KeyMenu => MenuAction::KeyMenu,
            MenuAction::KeyNext => MenuAction::KeyNext,
            MenuAction::KeyPrev => MenuAction::KeyPrev,
            MenuAction::KeyCut => MenuAction::KeyCut,
            MenuAction::KeyPaste => MenuAction::KeyPaste,
            MenuAction::SaveConfig => MenuAction::SaveConfig,
        }
    }
}

impl MenuItem {
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl InputDialog {
    pub fn new(prompt: &str, width: u16) -> Self {
        InputDialog {
            prompt: prompt.to_string(),
            width,
            input: String::new(),
        }
    }

    pub fn handle_input(&mut self, ch: char) -> bool {
        match ch {
            '\n' => true,
            '\x08' | '\x7f' => {
                self.input.pop();
                false
            },
            _ => {
                if !ch.is_control() {
                    self.input.push(ch);
                }
                false
            }
        }
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn show(&mut self) -> Option<String> {
        // Display dialog and get input
        Some(self.input.clone())
    }

    pub fn get_key(&self) -> i32 {
        // Get single keypress
        0 // Temporary implementation
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_prompt(&self) -> &str {
        &self.prompt
    }
}

pub fn create_main_menu() -> Menu {
    let mut menu = Menu::new("Main Menu");
    menu.add_item("File", MenuAction::None);
    menu.add_item("Edit", MenuAction::None);
    menu.add_item("Help", MenuAction::None);
    menu.add_item("Exit", MenuAction::Exit);
    menu
}

pub fn create_file_menu() -> Menu {
    let mut menu = Menu::new("File Menu");
    menu.add_item("Save", MenuAction::Save);
    menu.add_item("Save As...", MenuAction::SaveAs);
    menu.add_item("Close", MenuAction::Close);
    menu
}

pub fn create_keyboard_menu(_config: &EditorConfig) -> Menu {
    let mut menu = Menu::new("Keyboard Menu");
    menu.add_item("Exit Key", MenuAction::KeyExit);
    menu.add_item("Write Key", MenuAction::KeyWrite);
    menu.add_item("Search Key", MenuAction::KeySearch);
    menu.add_item("Menu Key", MenuAction::KeyMenu);
    menu.add_item("Next Key", MenuAction::KeyNext);
    menu.add_item("Previous Key", MenuAction::KeyPrev);
    menu.add_item("Cut Key", MenuAction::KeyCut);
    menu.add_item("Paste Key", MenuAction::KeyPaste);
    menu.add_item("Save Changes", MenuAction::SaveConfig);
    menu.add_item("Close", MenuAction::Close);
    menu
}

pub fn create_search_menu() -> Menu {
    let mut menu = Menu::new("Search Menu");
    menu.add_item("Find", MenuAction::Search);
    menu.add_item("Find Next", MenuAction::FindNext);
    menu.add_item("Replace", MenuAction::Replace);
    menu
}
