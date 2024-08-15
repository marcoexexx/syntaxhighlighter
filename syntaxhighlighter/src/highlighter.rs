use crate::token::Token;
use crate::tokenizer::LanguageTokenizer;

pub struct Highlighter {
    language: Box<dyn LanguageTokenizer>,
}

impl Highlighter {
    pub fn new(language: Box<dyn LanguageTokenizer>) -> Self {
        Highlighter { language }
    }

    pub fn highlight(&self, code: &str) -> Vec<Token> {
        self.language.tokenizer().tokenize(code)
    }
}
