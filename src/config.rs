#[derive(Debug, Default)]
pub struct Config {
    pub tab_width: usize,
    pub line_numbers: bool,
    pub syntax_highlight: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            tab_width: 4,
            line_numbers: true,
            syntax_highlight: true,
        }
    }
}
