use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;
use crate::error::runtime::RuntimeError;
use crate::parse::parser::{Node, NodeType};
use crate::runtime::frame::Frame;

pub struct Program<'a> {
    pub started_at: Instant,
    pub ast: Node,
    pub top_level_frame: Rc<RefCell<Frame<'a>>>,
    pub current_frame: Rc<RefCell<Frame<'a>>>
}


type RuntimeReturn = Option<RuntimeError>;



impl<'a> Program<'a> {
    pub fn new(ast: Node) -> Self {
        let frame = Rc::new(RefCell::new(Frame::new(None)));

        
        Self {
            started_at: Instant::now(),
            ast,
            top_level_frame: Rc::clone(&frame),
            current_frame: frame
        }
    }

    pub fn start(&self) -> RuntimeReturn {

        self.execute_entrypoint(&self.ast)?;

        None
    }


    fn execute_entrypoint(&self, node: &Node) -> RuntimeReturn {
        match node.node_type.as_ref() {
            NodeType::FunctionDefinition(def) => {
                self.execute_node(&def.body)
            },
            _ => Some(RuntimeError::expected_node_type("function definition", node.node_type.as_ref()))
        }
    }

    fn execute_node(&self, node: &Node) -> RuntimeReturn {
        match node.node_type.as_ref() {
            NodeType::Block(nodes ) => self.execute_block(nodes),
            _ => todo!("Node type not implemented in runtime")
        }
    }

    fn execute_block(&self, inner_nodes: &Vec<Node>) -> RuntimeReturn {
        for node in inner_nodes {
            self.execute_node(node)?;
        }

        None
    }

    fn start_new_frame(&self, inner_ast: &Node) {
        let frame = Rc::new(RefCell::new(Frame::new(Some(self.current_frame.borrow()))));
    }
    

    pub fn execute(&self) -> RuntimeReturn {
        let main_method = self.find_main_method();

        if main_method.is_none() {
            return Some(RuntimeError::new("No main method present in top-level context.".to_string()));
        }


        self.start()
    }

    fn find_main_method(&self) -> Option<&Node> {
        match self.ast.node_type.as_ref() {
            NodeType::Program(nodes) => {
                for node in nodes.iter() {
                    return match node.node_type.as_ref() {
                        NodeType::FunctionDefinition(definition) => {
                            if definition.name != "main" {
                                return None;
                            }

                            Some(&definition.body)
                        }
                        _ => continue
                    };
                }
                None
            }
            _ => None
        }
    }
}