#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Const(String),
    Identifier(String),
    FunctionName(String),
    Number(String),
    StringLiteral(String),
    Operator(String),
    Punctuation(String),
    Comment(String),
    Whitespace,
    EOF,
}
