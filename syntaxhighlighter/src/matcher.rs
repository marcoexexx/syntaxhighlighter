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
            Self::consume_while(chars, |c| c.is_whitespace());
            return Some(Token::Whitespace);
        }
        None
    }

    pub fn match_identifier_or_keyword(
        chars: &mut Peekable<std::str::Chars>,
        keywords: &[&str],
    ) -> Option<Token> {
        if chars.peek()?.is_alphabetic() || *chars.peek()? == '_' {
            let identifier = Self::consume_while(chars, |c| c.is_alphabetic() || c == '_');

            if keywords.contains(&identifier.as_ref()) {
                return Some(Token::Keyword(identifier));
            } else if identifier.chars().next().unwrap().is_uppercase() {
                return Some(Token::Const(identifier));
            } else {
                if let Some(&ch) = chars.peek() {
                    if ch == '(' {
                        return Some(Token::FunctionName(identifier));
                    }
                }

                return Some(Token::Identifier(identifier));
            }
        }

        None
    }

    pub fn match_number(chars: &mut Peekable<std::str::Chars>) -> Option<Token> {
        if chars.peek()?.is_digit(10) {
            let number = Self::consume_while(chars, |c| c.is_digit(10));
            return Some(Token::Number(number));
        }
        None
    }

    pub fn match_operator(
        chars: &mut Peekable<std::str::Chars>,
        operators: &[&str],
    ) -> Option<Token> {
        for &operator in operators {
            let mut temp_chars = chars.clone();
            let mut matched = true;

            for op_char in operator.chars() {
                if temp_chars.next() != Some(op_char) {
                    matched = false;
                    break;
                }
            }

            if matched {
                // Consume the operator charactors from the original iterator
                for _ in operator.chars() {
                    chars.next();
                }
                return Some(Token::Operator(operator.to_string()));
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

    pub fn match_string_literal(chars: &mut Peekable<std::str::Chars>) -> Option<Token> {
        if let Some(&ch) = chars.peek() {
            if ch == '"' || ch == '\'' || ch == '`' {
                let quote_type = ch;
                chars.next(); // Consume the opening quote

                let string_literal = Self::consume_while(chars, |c| c != quote_type);
                chars.next(); // Consume the closing quote

                return Some(Token::StringLiteral(string_literal));
            }
        }

        None
    }

    pub fn match_comment(chars: &mut Peekable<std::str::Chars>) -> Option<Token> {
        if let Some(&ch) = chars.peek() {
            if ch == '/' {
                chars.next();
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        // Single-line comment
                        chars.next();

                        let comment = Self::consume_while(chars, |c| c != '\n');
                        return Some(Token::Comment(comment));
                    } else if next_ch == '*' {
                        // Multi-line comment
                        chars.next();

                        let mut comment = String::new();
                        while let Some(c) = chars.next() {
                            if c == '*' && chars.peek() == Some(&'/') {
                                chars.next(); // Consume the closing '/'
                                break;
                            }
                            comment.push(c);
                        }
                        return Some(Token::Comment(comment));
                    }
                }
            }
        }

        None
    }
}
