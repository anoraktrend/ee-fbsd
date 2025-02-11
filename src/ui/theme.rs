use tui::style::Color;

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub selection: Color,
    pub comment: Color,
    pub string: Color,
    pub keyword: Color,
    pub function: Color,
    pub number: Color,
    pub type_name: Color,
    pub constant: Color,
    pub warning: Color,
    pub error: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(40, 44, 52),
            foreground: Color::Rgb(171, 178, 191),
            selection: Color::Rgb(62, 68, 81),
            comment: Color::Rgb(92, 99, 112),
            string: Color::Rgb(152, 195, 121),
            keyword: Color::Rgb(198, 120, 221),
            function: Color::Rgb(97, 175, 239),
            number: Color::Rgb(209, 154, 102),
            type_name: Color::Rgb(229, 192, 123),
            constant: Color::Rgb(224, 108, 117),
            warning: Color::Rgb(209, 154, 102),
            error: Color::Rgb(224, 108, 117),
        }
    }
}
