use crate::lexer::tokenizer::Token;
use crate::parse::parser::StatParser;

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub position: i128
}

impl ParserError {
    pub fn new(parser: &StatParser, message: String) -> Self {
        ParserError {
            message,
            position: parser.current_token.as_ref().unwrap_or(&Token::default()).start_pos
        }
    }
}