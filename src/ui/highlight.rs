use crate::ui::theme::{Theme, Color};
use syntect::highlighting as synhighlight;

pub fn convert_syntect_color(color: synhighlight::Color) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}

pub fn highlight_content(content: &str, theme: &Theme) -> Vec<(Color, String)> {
    let mut result = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        let color = match true {
            _ if trimmed.starts_with("//") => theme.comment.clone(),
            _ if trimmed.starts_with("\"") && trimmed.ends_with("\"") => theme.string.clone(),
            _ if trimmed.starts_with("fn ") || trimmed.ends_with("()") => theme.function.clone(),
            _ => theme.foreground.clone(),
        };
        result.push((color, line.to_string()));
    }
    
    result
}
