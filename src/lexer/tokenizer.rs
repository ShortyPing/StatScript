use crate::error::lexer::TokenizerError;
use crate::lexer::symbols::SymbolType;
use std::hint::black_box;
use std::ops::Sub;

#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub start_pos: i128,
    pub end_pos: i128,
    pub value: Option<String>,
    pub token_type: Option<TokenType>,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Number(bool),
    Symbol(SymbolType),
    String,
}

type TokenReturn = Result<Option<Token>, TokenizerError>;

// #[derive(Debug, Default)]
// pub enum TokenizerState {
//     #[default]
//     None,
//     String,
//     Number,
//     Identifier,
//     Finished,
// }

#[derive(Debug, Default)]
pub struct Tokenizer {
    pub content: String,
    pub cursor_position: i128,
    pub cursor_line: i128,
    // pub tokenizer_state: TokenizerState,
    pub buffer: String,
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        Tokenizer {
            content,
            buffer: String::default(),
            cursor_position: 0,
            cursor_line: 1,
            // tokenizer_state: TokenizerState::None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let r = self.content.chars().nth(self.cursor_position as usize);
        self.cursor_position += 1;
        r
    }

    pub fn next_token(&mut self) -> TokenReturn {
        self.reset();

        let mut token = Token::default();
        token.start_pos = self.cursor_position + 1;

        let c = match self.next_char() {
            Some(c) => c,
            None => return Ok(None),
        };

        match c {
            c if c.is_alphabetic() => {
                self.buffer.push(c);
                self.parse_identifier(token)
            }
            c if c.is_numeric() => {
                self.buffer.push(c);
                self.parse_number(token)
            }
            c if c == '"' => self.parse_string(token),
            c if c.is_whitespace() => self.next_token(),
            c => Err(TokenizerError::new(
                self,
                format!("Invalid char at beginning of token: {c}"),
            )),
        }
    }

    fn reset(&mut self) {
        self.buffer = String::default();
    }

    fn parse_identifier(&mut self, mut token: Token) -> TokenReturn {
        token.token_type = Some(TokenType::Identifier);
        while let Some(c) = self.next_char() {
            if !c.is_alphanumeric() {
                break;
            }

            self.buffer.push(c)
        }

        self.finish_token(&mut token);

        self.cursor_position -= 1; // move back in case we have e.g. print()

        Ok(Some(token))
    }

    fn parse_number(&mut self, mut token: Token) -> TokenReturn {
        let mut is_decimal = false;

        while let Some(c) = self.next_char() {
            if c.is_numeric() {
                self.buffer.push(c);
                continue;
            }

            if c == '.' {
                if is_decimal {
                    return Err(TokenizerError::new(
                        self,
                        "Cannot have more than one decimal in number.".into(),
                    ));
                }

                self.buffer.push(c);
                is_decimal = true;
                continue;
            }

            break;
        }

        token.token_type = Some(TokenType::Number(is_decimal));

        self.finish_token(&mut token);

        self.cursor_position -= 1;
        Ok(Some(token))
    }

    fn parse_string(&mut self, mut token: Token) -> TokenReturn {
        let mut is_escaped = false;
        while let Some(c) = self.next_char() {
            if c == '"' {
                if !is_escaped {
                    break;
                }

                is_escaped = false;
            }

            if c == '\\' {
                if !is_escaped {
                    is_escaped = true;
                    continue;
                }

                is_escaped = false;
            }

            self.buffer.push(c);
        }

        token.token_type = Some(TokenType::String);
        self.finish_token(&mut token);

        Ok(Some(token))
    }

    fn finish_token(&mut self, token: &mut Token) {
        token.end_pos = self.cursor_position - 1; // -1 because we exclude the overflowed char
        token.value = Some(self.buffer.clone());
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::lexer::tokenizer::{Token, TokenType, Tokenizer};

    #[test]
    fn test_identifier() {
        let mut tokenizer = Tokenizer::new("abc cba".into());

        let token = tokenizer.next_token().unwrap().unwrap();

        assert_eq!(
            token,
            Token {
                start_pos: 1,
                end_pos: 3,
                value: Some("abc".into()),
                token_type: Some(TokenType::Identifier),
            }
        );

        let token1 = tokenizer.next_token().unwrap().unwrap();

        assert_eq!(
            token1,
            Token {
                start_pos: 5,
                end_pos: 7,
                value: Some("cba".into()),
                token_type: Some(TokenType::Identifier),
            }
        )
    }
}
