use crate::mir::types::{Type};
use std::collections::HashMap;
use crate::mir::expressions::literal::{Object};
use std::rc::Rc;

pub mod literal;

pub mod binary;

pub mod function;

pub mod variable;

pub mod casting;

pub mod statements;

#[derive(Clone)]
pub struct Environment {
    pub scope: HashMap<String, Object>,
    pub function_registry: Rc<HashMap<FunctionSig, Box<dyn Expression>>>,
}

pub trait Expression {
    fn get_type(&self, environment: &Environment) -> Type;
    fn eval(&self, environment: &Environment) -> Object;
}

#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FunctionSig {
    pub name: String,
    pub args: Vec<Type>,
    pub return_type: Type
}