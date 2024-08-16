use std::iter::Peekable;

use crate::token::Token;

pub struct TokenPattern {
    pub matcher: fn(&mut Peekable<std::str::Chars>) -> Option<Token>,
}

pub struct Tokenizer {
    pub patterns: Vec<TokenPattern>,
}

impl Tokenizer {
    pub fn new(patterns: Vec<TokenPattern>) -> Self {
        Tokenizer { patterns }
    }

    pub fn tokenize(&self, code: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = code.chars().peekable();

        while let Some(_) = chars.peek() {
            let mut matched = false;

            for pattern in &self.patterns {
                if let Some(token) = (pattern.matcher)(&mut chars) {
                    tokens.push(token);
                    matched = true;
                    break;
                }
            }

            if !matched {
                chars.next(); // Consume any unrecognized character to avoid infinite loop
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}

pub trait LanguageTokenizer {
    fn tokenizer(&self) -> Tokenizer;
}
