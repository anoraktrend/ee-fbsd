use crate::ui::theme::Theme;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use regex::Regex;  // Changed this import
use tui::style::Color;
use std::path::Path;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    theme: Option<Theme>,
    numbers_regex: Option<Regex>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let numbers_regex = Regex::new(r"\b\d+\b").ok();

        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            theme: None,
            numbers_regex,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = Some(theme);
    }

    pub fn highlight_line<'a>(&self, line: &'a str, extension: Option<&str>) -> Vec<(Color, &'a str)> {
        let default_theme = Theme::default();
        let theme = self.theme.as_ref().unwrap_or(&default_theme);
        
        let mut ranges = Vec::new();
        
        if let Some(comment_pos) = line.find("//") {
            if comment_pos > 0 {
                self.highlight_code_segment(&line[..comment_pos], extension, theme, &mut ranges);
            }
            ranges.push((theme.syntax.comment, &line[comment_pos..]));
            return ranges;
        }

        self.highlight_code_segment(line, extension, theme, &mut ranges);
        
        if ranges.is_empty() && !line.is_empty() {
            ranges.push((theme.ui.foreground, line));
        }

        ranges
    }

    fn highlight_code_segment<'a>(
        &self,
        text: &'a str,
        _extension: Option<&str>,
        theme: &Theme,
        ranges: &mut Vec<(Color, &'a str)>
    ) {
        if let Some(_regex) = &self.numbers_regex {
            // TODO: Implement regex-based highlighting
            ranges.push((theme.syntax.number, text));
        } else {
            ranges.push((theme.ui.foreground, text));
        }
    }

    fn get_extension(path: &Path) -> Option<&str> {
        path.extension()
            .and_then(|os_str| os_str.to_str())
    }
}
