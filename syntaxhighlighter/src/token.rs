#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Operator(String),
    Punctuation(String),
    Comment(String),
    Whitespace,
    EOF,
}
