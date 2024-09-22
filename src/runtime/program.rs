use std::ops::Deref;
use std::time::Instant;
use crate::error::runtime::RuntimeError;
use crate::parse::parser::{Node, NodeType};

pub struct Program<'a> {
    pub started_at: Instant,
    pub ast: &'a Node
}

pub struct Thread<'a> {
    pub name: String,
    pub ast: &'a Node
}

impl<'a> Thread<'a> {
    
    pub fn start(& self) -> Option<RuntimeError> {
        println!("{:#?}", self.ast);
        None
    }
}


impl<'a> Program<'a> {

    pub fn new(ast: &'a Node) -> Self {
        Self {
            started_at: Instant::now(),
            ast
        }
    }

    pub fn execute(&mut self) -> Option<RuntimeError> {
        let main_method = self.find_main_method();

        if main_method.is_none() {
            return Some(RuntimeError::new("No main method present in top-level context.".to_string()))
        }

        let main_thread = Some(Thread {
            name: "main".to_string(),
            ast: main_method.unwrap()
        });

        

        main_thread?.start()
    }

    fn find_main_method(&mut self) -> Option<&'a Node> {
        match self.ast.node_type.as_ref() {
            NodeType::Program(nodes) => {

                for node in nodes.iter() {
                    return match node.node_type.as_ref()  {
                        NodeType::FunctionDefinition(definition) => {
                            if definition.name != "main" {
                                return None
                            }

                            Some(&definition.body)
                        }
                        _ => continue
                    }
                }
                None
            },
            _ => None
        }

    }
}