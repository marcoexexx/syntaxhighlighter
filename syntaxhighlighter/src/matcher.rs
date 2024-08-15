use std::iter::Peekable;

use crate::token::Token;

pub struct PatternMatcher;

impl PatternMatcher {
    fn consume_while<F>(chars: &mut Peekable<std::str::Chars>, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while let Some(&ch) = chars.peek() {
            if condition(ch) {
                result.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        result
    }

    pub fn match_whitespace(chars: &mut Peekable<std::str::Chars>) -> Option<Token> {
        if chars.peek()?.is_whitespace() {
            PatternMatcher::consume_while(chars, |c| c.is_whitespace());
            return Some(Token::Whitespace);
        }
        None
    }

    pub fn match_identifier_or_keyword(
        chars: &mut Peekable<std::str::Chars>,
        keywords: &[&str],
    ) -> Option<Token> {
        if chars.peek()?.is_alphabetic() || *chars.peek()? == '_' {
            let identifier = PatternMatcher::consume_while(chars, |c| c.is_alphabetic() || c == '_');

            if keywords.contains(&identifier.as_ref()) {
                return Some(Token::Keyword(identifier));
            } else {
                return Some(Token::Identifier(identifier));
            }
        }

        None
    }

    pub fn match_number(chars: &mut Peekable<std::str::Chars>) -> Option<Token> {
        if chars.peek()?.is_digit(10) {
            let number = PatternMatcher::consume_while(chars, |c| c.is_digit(10));
            return Some(Token::Number(number));
        }
        None
    }

    pub fn match_operator(
        chars: &mut Peekable<std::str::Chars>,
        operator: &[char],
    ) -> Option<Token> {
        if let Some(&ch) = chars.peek() {
            if operator.contains(&ch) {
                chars.next();
                return Some(Token::Operator(ch.to_string()));
            }
        }

        None
    }

    pub fn match_punctuation(
        chars: &mut Peekable<std::str::Chars>,
        punctuations: &[char],
    ) -> Option<Token> {
        if let Some(&ch) = chars.peek() {
            if punctuations.contains(&ch) {
                chars.next();
                return Some(Token::Punctuation(ch.to_string()));
            }
        }

        None
    }
}
