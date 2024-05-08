use crate::lexer::tokenizer::Tokenizer;

#[derive(Debug)]
pub struct TokenizerError {
    pub position: i128,
    pub line: i128,
    pub message: String,
}

impl TokenizerError {
    pub fn new(tokenizer: &Tokenizer, message: String) -> Self {
        TokenizerError {
            position: tokenizer.cursor_position,
            line: tokenizer.cursor_line,
            message,
        }
    }
}
