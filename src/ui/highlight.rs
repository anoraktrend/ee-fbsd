use tui::style::Color;

pub struct SyntaxHighlighter {
    js_keywords: Vec<&'static str>,
    rust_keywords: Vec<&'static str>,
    numbers_regex: regex::Regex,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            js_keywords: vec![
                "function", "const", "let", "var", "if", "else", "return", "class",
                "extends", "new", "for", "while", "break", "continue", "import", "export"
            ],
            rust_keywords: vec![
                "fn", "let", "const", "mut", "pub", "use", "mod", "struct", "enum",
                "impl", "trait", "where", "match", "if", "else", "loop", "while", "for",
                "in", "return", "self", "Self", "super", "async", "await", "dyn"
            ],
            numbers_regex: regex::Regex::new(r"\b\d+\b").unwrap(),
        }
    }

    pub fn highlight_line<'a>(&self, line: &'a str, extension: Option<&str>) -> Vec<(Color, &'a str)> {
        let mut result = Vec::new();
        let mut last_pos = 0;

        // Handle comments first
        if let Some(comment_pos) = line.find("//") {
            if comment_pos > 0 {
                self.highlight_code_segment(&line[..comment_pos], extension, &mut result);
            }
            result.push((Color::DarkGray, &line[comment_pos..]));
            return result;
        }

        // If no comments, highlight the whole line
        self.highlight_code_segment(line, extension, &mut result);

        if result.is_empty() && !line.is_empty() {
            result.push((Color::Reset, line));
        }

        result
    }

    fn highlight_code_segment<'a>(
        &self,
        text: &'a str,
        extension: Option<&str>,
        result: &mut Vec<(Color, &'a str)>
    ) {
        let mut last_pos = 0;
        let keywords = match extension {
            Some("rs") => &self.rust_keywords,
            Some("js") | Some("ts") => &self.js_keywords,
            _ => &[],
        };

        // Split the text into words and handle each separately
        for (start, word) in text.split_word_bounds().enumerate() {
            let word_trimmed = word.trim();
            
            // Skip empty or whitespace
            if word_trimmed.is_empty() {
                result.push((Color::Reset, word));
                continue;
            }

            // Highlight strings
            if (word.starts_with('"') && word.ends_with('"')) ||
               (word.starts_with('\'') && word.ends_with('\'')) {
                result.push((Color::Green, word));
                continue;
            }

            // Highlight numbers
            if self.numbers_regex.is_match(word) {
                result.push((Color::Yellow, word));
                continue;
            }

            // Highlight keywords
            if keywords.contains(&word_trimmed) {
                result.push((Color::Magenta, word));
                continue;
            }

            // Default color for other text
            result.push((Color::Reset, word));
        }
    }
}
