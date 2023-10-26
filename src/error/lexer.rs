use crate::lexer::tokenizer::Tokenizer;

#[derive(Debug)]
pub struct TokenizerError {
    position: i128,
    line: i128,
    message: String,
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
