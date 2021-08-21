use crate::mir::types::{Type};
use std::collections::HashMap;
use crate::mir::expressions::literal::{Object};
use std::rc::Rc;
use std::cell::RefCell;

pub mod literal;

pub mod binary;

pub mod function;

pub mod variable;

pub mod casting;

pub mod statements;

#[derive(Clone)]
pub struct ExpressionLevelEnvironment {
    pub scope: HashMap<String, Object>,
    pub function_registry: Rc<RefCell<HashMap<FunctionSig, Box<dyn Expression>>>>,
}

pub trait Expression {
    fn get_type(&self, environment: &ExpressionLevelEnvironment) -> Type;
    fn eval(&self, environment: &ExpressionLevelEnvironment) -> Object;
}

#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FunctionSig {
    pub name: String,
    pub args: Vec<Type>,
    pub return_type: Type
}