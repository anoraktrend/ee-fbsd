use syntect::highlighting::{ThemeSet, Theme};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::util::LinesWithEndings;
use tui::style::{Color, Style};
use std::path::Path;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    current_theme: String,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        
        Self {
            syntax_set,
            theme_set,
            current_theme: "base16-ocean.dark".to_string(),
        }
    }

    pub fn highlight_line(&self, line: &str, extension: Option<&str>) -> Vec<(Color, &str)> {
        let syntax = match extension {
            Some(ext) => self.syntax_set.find_syntax_by_extension(ext)
                .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text()),
            None => self.syntax_set.find_syntax_plain_text()
        };

        let mut h = syntect::easy::HighlightLines::new(
            syntax,
            &self.theme_set.themes[&self.current_theme]
        );

        let ranges = h.highlight_line(line, &self.syntax_set)
            .unwrap_or_default();

        ranges.into_iter()
            .map(|(style, text)| {
                let color = Color::Rgb(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b
                );
                (color, text)
            })
            .collect()
    }

    pub fn get_extension(path: &Path) -> Option<&str> {
        path.extension()
            .and_then(|os_str| os_str.to_str())
    }
}
