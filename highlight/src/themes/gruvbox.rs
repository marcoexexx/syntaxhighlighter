use crate::theme::{Color, Theme};

pub fn gruvbox_theme() -> Theme {
    Theme {
        keyword: Color::RGB((251, 73, 52)),         // Red (#fb4934)
        r#const: Color::RGB((254, 128, 25)),        // Orange (#fe8019)
        identifier: Color::RGB((250, 189, 47)),     // Yellow (#fabd2f)
        function_name: Color::RGB((142, 192, 124)), // Aqua/Green (#8ec07c)
        number: Color::RGB((211, 134, 155)),        // Purple (#d3869b)
        string_literal: Color::RGB((184, 187, 38)), // Green (#b8bb26)
        operator: Color::RGB((131, 165, 152)),      // Blue (#83a598)
        punctuation: Color::RGB((235, 219, 178)),   // Light (#ebdbb2)
        comment: Color::RGB((146, 131, 116)),       // Gray (#928374)
    }
}
