use crate::buffer::Buffer;
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::SyntaxSet,
};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

use super::theme::Theme;

pub fn highlight_content<'a>(buffer: &'a Buffer, theme: &'a Theme) -> Vec<Spans<'a>> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    let syntax = buffer.syntax_name()
        .and_then(|name| ss.find_syntax_by_extension(name))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    buffer.lines().iter().map(|line| {
        let line_str = line.as_str();
        let regions = match h.highlight_line(line_str, &ss) {
            Ok(regions) => regions,
            Err(_) => vec![(syntect::highlighting::Style::default(), line_str)]
        };
        
        let spans: Vec<Span> = regions.into_iter().map(|(style, text)| {
            let color = match style.foreground {
                fg if fg == syntect::highlighting::Color::WHITE => theme.foreground,
                fg => convert_color(fg),
            };
            Span::styled(text.to_string(), Style::default().fg(color))
        }).collect();
        Spans::from(spans)
    }).collect()
}

fn convert_color(color: syntect::highlighting::Color) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}
