use std::fmt::format;
use std::process::id;
use std::rc::Rc;
use clap::builder::Str;
use crate::error::parser::ParserError;
use crate::lexer::symbols::SymbolType;
use crate::lexer::symbols::SymbolType::{AtSign, BraceLeft, BraceRight, ParenthesisLeft, ParenthesisRight};
use crate::lexer::tokenizer::{Token, Tokenizer, TokenType};

type ParserReturn = Result<Node, ParserError>;

#[derive(Debug)]
pub enum NodeType {
    Program(Vec<Node>),
    Block(Vec<Node>),
    StringLiteral(String),
    Int8Literal(u8),
    Int16Literal(u16),
    Int32Literal(u32),
    Int64Literal(u64),
    Int128Literal(u128),
    DoubleLiteral(f64),
    ReturnExpression(Node),
    UnaryNode(UnaryType, Node),
    FunctionDefinition(FunctionDefinition),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, Clone)]
pub enum UnaryType {
    Not
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub signature: Vec<FunctionParameter>,
    pub return_type: String,
    pub body: Node,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: Option<String>,
    pub value: Node,
}


#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub param_type: String,
    pub param_name: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: Rc<NodeType>,
}

pub struct StatParser {
    tokenizer: Tokenizer,
    ast: Node,
    pub current_token: Option<Token>,
}

impl StatParser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer,
            ast: Node {
                node_type: Rc::new(NodeType::Program(Vec::new()))
            },
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> ParserReturn {
        let program = self.parse_program()?;


        self.ast.node_type = Rc::new(program);
        Ok(self.ast.clone())
    }

    fn parse_program(&mut self) -> Result<NodeType, ParserError> {
        let mut nodes: Vec<Node> = Vec::new();


        while let Some(token) = self.next_token()? {
            self.current_token = Some(token.clone());
            return match &token.token_type {
                None => {
                    Err(ParserError::new(self, "None token type found... internal parsing error".into()))
                }
                Some(t) => {
                    match t {
                        TokenType::Identifier => {
                            let identifier_value = self.unwrap_guaranteed_value(token.value)?;

                            if identifier_value == "func" {
                                nodes.push(self.parse_function_definition()?);
                                continue;
                            }

                            Err(ParserError::new(self, format!("Unexpected identifier '{}'", identifier_value)))
                        }
                        tt => {
                            Err(ParserError::new(self, format!("Invalid token type {:?} as top-level statement", tt)))
                        }
                    }
                }
            };
        }


        Ok(NodeType::Program(nodes))
    }

    fn next_token(&mut self) -> Result<Option<Token>, ParserError> {
        let t = self.tokenizer.next_token().map_err(|e| ParserError::new(self, format!("Tokenizer error: {}:{} {}", e.position, e.line, e.message)))?;


        self.current_token.clone_from(&t);

        Ok(t)
    }

    fn next_token_expect(&mut self) -> Result<Token, ParserError> {
        match self.next_token()? {
            None => Err(ParserError::new(self, "Expected token".into())),
            Some(t) => {
                Ok(t)
            }
        }
    }

    fn unwrap_guaranteed_value(&self, o: Option<String>) -> Result<String, ParserError> {
        o.clone().ok_or(ParserError::new(self, "Identifier has None value... internal parsing error".into()))
    }

    fn parse_function_definition(&mut self) -> ParserReturn {
        let mut body: Option<Node> = None;
        let mut name: Option<String> = None;
        let mut signature: Vec<FunctionParameter> = Vec::new();
        let mut return_type: Option<String> = None;


        let name_token = self.next_token_expect()?;

        if name_token.token_type.unwrap() != TokenType::Identifier {
            return Err(ParserError::new(self, "Expected identifier as function name".into()));
        }


        name = Some(self.unwrap_guaranteed_value(name_token.value)?);

        if self.next_token_expect()?.token_type.unwrap() != TokenType::Symbol(ParenthesisLeft) {
            return Err(ParserError::new(self, "Expected '(' after function name".into()));
        }

        if self.next_token_expect()?.token_type.unwrap() != TokenType::Symbol(ParenthesisRight) {
            todo!()
        }

        if self.next_token_expect()?.token_type.unwrap() == TokenType::Symbol(AtSign) {
            let return_token = self.next_token_expect()?;
            if return_token.token_type.unwrap() != TokenType::Identifier {
                return Err(ParserError::new(self, "Expected identifier as function return type".into()));
            }

            return_type = Some(self.unwrap_guaranteed_value(return_token.value)?);

            self.next_token_expect()?;
        }

        if self.current_token.clone().unwrap().token_type.unwrap() != TokenType::Symbol(BraceLeft) {
            return Err(ParserError::new(self, "Expected '{' as begin of function body".into()));
        }

        body = Some(self.parse_block()?);

        Ok(Node {
            node_type: Rc::new(NodeType::FunctionDefinition(FunctionDefinition {
                name: name.unwrap(),
                signature,
                return_type: return_type.unwrap_or("Nothing".into()),
                body: body.unwrap(),
            }))
        })
    }

    fn parse_expression(&mut self) -> ParserReturn {
        let tok = self.next_token_expect()?;

        return match tok.token_type.as_ref().unwrap() {
            TokenType::String => {
                Ok(Node {
                    node_type: Rc::new(NodeType::StringLiteral(self.unwrap_guaranteed_value(tok.value)?))
                })
            }
            TokenType::Number(b) => {
                let value = self.unwrap_guaranteed_value(tok.value)?;

                if *b {
                    let numeric: f64 = value.parse().map_err(|_| ParserError::new(self, format!("Literal {value} is not a valid double.")))?;

                    return Ok(Node {
                        node_type: Rc::new(NodeType::DoubleLiteral(numeric))
                    });
                }

                let numeric: i128 = value.parse().map_err(|_| ParserError::new(self, format!("Number {value} is an invalid number")))?;
                let log = numeric.ilog2() + 1;

                if log <= 8 {
                    return Ok(Node {
                        node_type: Rc::new(NodeType::Int8Literal(u8::try_from(numeric).unwrap()))
                    });
                } else if log <= 16 {
                    return Ok(Node {
                        node_type: Rc::new(NodeType::Int16Literal(u16::try_from(numeric).unwrap()))
                    });
                } else if log <= 32 {
                    return Ok(Node {
                        node_type: Rc::new(NodeType::Int32Literal(u32::try_from(numeric).unwrap()))
                    });
                } else if log <= 64 {
                    return Ok(Node {
                        node_type: Rc::new(NodeType::Int64Literal(u64::try_from(numeric).unwrap()))
                    });
                } else if log <= 128 {
                    return Ok(Node {
                        node_type: Rc::new(NodeType::Int128Literal(u128::try_from(numeric).unwrap()))
                    });
                } else {
                    return Err(ParserError::new(self, "You've bent the universe... or my memory idk".into()));
                }
            }
            TokenType::Symbol(BraceLeft) => {
                Ok(self.parse_block()?)
            }
            _ => Err(ParserError::new(self, format!("Invalid token {tok:?} as start of an expression.")))
        };
    }

    fn parse_variable_declaration(&mut self) -> ParserReturn {
        let err: String = "Syntax error in variable declaration. Example: 'set<i32> num <- 3".into();

        if self.next_token_expect()?.token_type.unwrap() != TokenType::Symbol(SymbolType::TagLeft) {
            return Err(ParserError::new(self, err));
        }

        let type_token = self.next_token_expect()?;
        if type_token.token_type.unwrap() != TokenType::Identifier {
            return Err(ParserError::new(self, "Expected identifier as variable type".into()));
        }

        if self.next_token_expect()?.token_type.unwrap() != TokenType::Symbol(SymbolType::TagRight) {
            return Err(ParserError::new(self, err));
        }

        let name_token = self.next_token_expect()?;

        if name_token.token_type.unwrap() != TokenType::Identifier {
            return Err(ParserError::new(self, "Expected identifier as variable name".into()));
        }

        if self.next_token_expect()?.token_type.unwrap() != TokenType::Symbol(SymbolType::TagLeft) {
            return Err(ParserError::new(self, err));
        }

        if self.next_token_expect()?.token_type.unwrap() != TokenType::Symbol(SymbolType::Minus) {
            return Err(ParserError::new(self, err));
        }

        let value = self.parse_expression()?;

        let var_type = self.unwrap_guaranteed_value(type_token.value)?;

        let mut t = Some(var_type.clone());

        if var_type == "inherit" {
            t = None;
        }

        Ok(Node {
            node_type: Rc::new(NodeType::VariableDeclaration(VariableDeclaration {
                name: self.unwrap_guaranteed_value(name_token.value)?,
                variable_type: t,
                value,
            }))
        })
    }


    fn parse_block(&mut self) -> ParserReturn {
        let mut nodes: Vec<Node> = Vec::new();

        let mut closed = false;

        while let Some(token) = self.next_token()? {
            if token.token_type.as_ref().unwrap() == &TokenType::Symbol(BraceRight) {
                closed = true;
                break;
            }


            let token_value = self.unwrap_guaranteed_value(token.value)?;
            if token_value == "set" {
                nodes.push(self.parse_variable_declaration()?);
                continue;
            }

            if token_value == "return" {
                nodes.push(Node {
                    node_type: Rc::new(NodeType::ReturnExpression(self.parse_expression()?))
                });
                continue;
            }

            if token.token_type == Some(TokenType::Symbol(BraceLeft)) {
                nodes.push(self.parse_block()?);
                continue;
            }


            return Err(ParserError::new(self, format!("Unexpected token {token_value}")));
        }

        if !closed {
            return Err(ParserError::new(self, "Expected block closure".into()));
        }

        Ok(Node {
            node_type: Rc::new(NodeType::Block(nodes))
        })
    }
}