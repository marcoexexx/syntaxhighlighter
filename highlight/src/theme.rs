use tokenizer::token::Token;

pub trait ApplyTheme {
    fn apply_theme(&self, ansi_color: &str, ident: &str, full_code: &mut String);
    fn apply_theme_and_render(&self, theme: &Theme, code: &mut String);
}

#[derive(Debug)]
pub enum Color {
    RGB((u8, u8, u8)),
    Hex(String),
}

impl Color {
    pub fn to_ansi(&self, foreground: bool) -> String {
        let offset = if foreground { "38" } else { "48" };

        match self {
            Color::RGB((r, g, b)) => format!("\x1b[{offset};2;{};{};{}m", r, g, b),
            Color::Hex(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Theme {
    pub keyword: Color,
    pub r#const: Color,
    pub identifier: Color,
    pub function_name: Color,
    pub number: Color,
    pub string_literal: Color,
    pub operator: Color,
    pub punctuation: Color,
    pub comment: Color,
}

impl ApplyTheme for Token {
    fn apply_theme(&self, ansi_color: &str, ident: &str, full_code: &mut String) {
        *full_code = full_code.replace(ident, &format!("{}{}\x1b[0m", ansi_color, ident))
    }

    fn apply_theme_and_render(&self, theme: &Theme, code: &mut String) {
        match self {
            Self::Keyword(ident) => self.apply_theme(&theme.keyword.to_ansi(true), ident, code),
            Self::Const(ident) => self.apply_theme(&theme.r#const.to_ansi(true), ident, code),
            Self::Identifier(ident) => {
                self.apply_theme(&theme.identifier.to_ansi(true), ident, code)
            }
            // Self::FunctionName(ident) => {
            //     self.apply_theme(&theme.function_name.to_ansi(true), ident, code)
            // }
            Self::Number(ident) => self.apply_theme(&theme.number.to_ansi(true), ident, code),
            Self::StringLiteral(ident) => {
                self.apply_theme(&theme.string_literal.to_ansi(true), ident, code)
            }
            Self::Operator(ident) => self.apply_theme(&theme.operator.to_ansi(true), ident, code),
            Self::Comment(ident) => self.apply_theme(&theme.comment.to_ansi(true), ident, code),
            // Self::Punctuation(ident) => {
            //     self.apply_theme(&theme.punctuation.to_ansi(true), ident, code)
            // }
            _ => {}
        }
    }
}
