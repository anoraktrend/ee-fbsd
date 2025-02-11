use ncurses::*;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
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

impl Theme {
    pub fn apply(&self) {
        // Initialize color pairs for ncurses
        start_color();
        use_default_colors();

        // Map theme colors to ncurses color pairs
        let colors = [
            (1, &self.foreground),
            (2, &self.comment),
            (3, &self.string),
            (4, &self.keyword),
            (5, &self.function),
            (6, &self.number),
            (7, &self.type_name),
            (8, &self.constant),
            (9, &self.warning),
            (10, &self.error),
        ];

        for (i, color) in colors.iter() {
            // Create custom color
            init_color(*i as i16, 
                (color.r as i16) * 1000 / 255,
                (color.g as i16) * 1000 / 255,
                (color.b as i16) * 1000 / 255);

            // Create color pair with background
            init_pair(*i as i16, *i as i16, -1);
        }

        // Set up selection highlighting
        init_color(11, 
            (self.selection.r as i16) * 1000 / 255,
            (self.selection.g as i16) * 1000 / 255,
            (self.selection.b as i16) * 1000 / 255);
        init_pair(11, -1, 11);
    }

    pub fn get_color_pair(&self, style: &str) -> i16 {
        match style {
            "normal" => 1,
            "comment" => 2,
            "string" => 3,
            "keyword" => 4,
            "function" => 5,
            "number" => 6,
            "type" => 7,
            "constant" => 8,
            "warning" => 9,
            "error" => 10,
            "selection" => 11,
            _ => 1,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color { r: 0, g: 0, b: 0 },
            foreground: Color { r: 208, g: 208, b: 208 },
            selection: Color { r: 64, g: 64, b: 64 },
            comment: Color { r: 95, g: 135, b: 95 },
            string: Color { r: 204, g: 147, b: 147 },
            keyword: Color { r: 207, g: 106, b: 76 },
            function: Color { r: 207, g: 181, b: 59 },
            number: Color { r: 181, g: 137, b: 0 },
            type_name: Color { r: 179, g: 157, b: 219 },
            constant: Color { r: 181, g: 137, b: 0 },
            warning: Color { r: 231, g: 140, b: 69 },
            error: Color { r: 204, g: 0, b: 0 },
        }
    }
}
