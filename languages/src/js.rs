use tokenizer::matcher::PatternMatcher;
use tokenizer::tokenizer::{LanguageTokenizer, TokenPattern, Tokenizer};

pub struct JavaScriptTokenizer;

impl LanguageTokenizer for JavaScriptTokenizer {
    fn tokenizer(&self) -> Tokenizer {
        Tokenizer::new(vec![
            TokenPattern {
                matcher: PatternMatcher::match_comment,
            },
            TokenPattern {
                matcher: PatternMatcher::match_whitespace,
            },
            TokenPattern {
                matcher: |chars| {
                    PatternMatcher::match_identifier_or_keyword(
                        chars,
                        &vec![
                            "break",
                            "case",
                            "catch",
                            "class",
                            "const",
                            "continue",
                            "debugger",
                            "default",
                            "delete",
                            "do",
                            "else",
                            "export",
                            "extends",
                            "finally",
                            "for",
                            "function",
                            "if",
                            "import",
                            "in",
                            "instanceof",
                            "let",
                            "new",
                            "return",
                            "super",
                            "switch",
                            "this",
                            "throw",
                            "try",
                            "typeof",
                            "var",
                            "void",
                            "while",
                            "with",
                            "yield",
                            "async",
                            "await",
                        ],
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
                        &vec![
                            "+", "-", "*", "/", "%", "=", "==", "===", "!=", "!==", ">", "<", ">=",
                            "<=", "&&", "||", "!", "??", "?.", "&", "|", "^", "~", "<<", ">>",
                            ">>>", "+=", "-=", "*=", "/=", "%=", "++", "--", "=>",
                        ],
                    )
                },
            },
            TokenPattern {
                matcher: |chars| {
                    PatternMatcher::match_punctuation(
                        chars,
                        &vec![';', ',', '.', '(', ')', '{', '}', '[', ']', ':', '?'],
                    )
                },
            },
            TokenPattern {
                matcher: PatternMatcher::match_string_literal,
            },
        ])
    }
}
