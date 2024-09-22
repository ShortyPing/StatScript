use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Frame<'a> {
    pub parent_frame: Option<&'a Frame<'a>>,
    pub variables: HashMap<String, Rc<dyn Any>>
}

impl<'a> Frame<'a> {
    
    fn new(parent_frame: Option<&'a Frame<'a>>) -> Self {
        Self {
            parent_frame,
            variables: Default::default(),
        }
    }

    fn find_variable(&self, name: String) -> Option<Rc<dyn Any>> {
        let mut next_frame: Option<&Self> = Some(self);

        while next_frame.is_some() {
            let variable = next_frame?.variables.get(&name);
            
            match variable { 
                Some(var) => {
                    return Some(var.clone())
                },
                None => {
                    next_frame = next_frame?.parent_frame;
                    continue
                }
            }
        }

        None
    }
}