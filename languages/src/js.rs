use syntaxhighlighter::matcher::PatternMatcher;
use syntaxhighlighter::tokenizer::{LanguageTokenizer, TokenPattern, Tokenizer};

pub struct JavaScriptTokenizer;

impl LanguageTokenizer for JavaScriptTokenizer {
    fn tokenizer(&self) -> Tokenizer {
        Tokenizer::new(vec![
            TokenPattern {
                matcher: PatternMatcher::match_whitespace,
            },
            TokenPattern {
                matcher: |chars| {
                    PatternMatcher::match_identifier_or_keyword(
                        chars,
                        &vec!["function", "let", "var", "const", "if", "else", "return"],
                    )
                },
            },
            TokenPattern {
                matcher: PatternMatcher::match_number,
            },
            TokenPattern {
                matcher: |chars| {
                    PatternMatcher::match_operator(
                        chars,
                        &vec!['+', '-', '*', '/', '=', '!', '<', '>'],
                    )
                },
            },
            TokenPattern {
                matcher: |chars| {
                    PatternMatcher::match_punctuation(
                        chars,
                        &vec![';', ',', '.', '(', ')', '{', '}'],
                    )
                },
            },
        ])
    }
}
