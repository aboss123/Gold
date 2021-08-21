use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::mir::expressions::{Expression, ExpressionLevelEnvironment, FunctionSig};
use crate::mir::expressions::literal::Object;
use crate::mir::types::Type;

pub struct BlockLevelEnvironment {
    pub function_registry: Rc<RefCell<HashMap<FunctionSig, Box<dyn Expression>>>>,
    pub scopes: Vec<HashMap<String, Object>>,
}

impl BlockLevelEnvironment {
    fn get_expression_environment(&self) -> ExpressionLevelEnvironment {
        let scope = HashMap::new();

        for s in self.scopes.iter() {
            // todo
        }
    }
}

pub trait Statement {
    fn eval(&self, environment: &mut BlockLevelEnvironment) -> Option<Object>;
}

pub struct Binding {
    pub ident: String,
    pub child: Box<dyn Expression>,
}

pub struct Print {
    pub output: Box<dyn Expression>,
}

impl Statement for Binding {
    fn eval(&self, environment: &mut BlockLevelEnvironment) -> Option<Object> {
        let obj = self.child.eval()
        let entry = environment.scopes.last_mut().unwrap().insert(self.ident.clone(), );



        None
    }
}

pub struct Block {
    pub statements: Vec<Box<dyn Statement>>,
}