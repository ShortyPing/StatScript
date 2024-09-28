use crate::parse::parser::NodeType;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
    
    pub fn expected_node_type(node_type: &str, actual_type: &NodeType) -> Self {
        Self::new(format!("Internal error! Expected node of type '{node_type}' but got '{actual_type:?}'"))
    }
}