use tokenizer::token::Token;
use tokenizer::tokenizer::LanguageTokenizer;

pub mod theme;
pub mod themes;

pub struct Highlighter<'a> {
    pub code: &'a str,
    pub language: Box<dyn LanguageTokenizer>,
}

impl<'a> Highlighter<'a> {
    pub fn new(code: &'a str, language: Box<dyn LanguageTokenizer>) -> Self {
        Highlighter { language, code }
    }

    pub fn highlight(&self) -> Vec<Token> {
        self.language.tokenizer().tokenize(self.code)
    }
}
