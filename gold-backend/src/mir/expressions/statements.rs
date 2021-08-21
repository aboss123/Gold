use crate::mir::types::Type;
use crate::mir::expressions::{Expression, Environment};
use crate::mir::expressions::literal::Object;

pub trait Statement {
    fn eval(&self, environment: &mut Environment) -> Option<Object>;
}

pub struct Binding {
    pub ident: String,
    pub child: Box<dyn Expression>
}

pub struct Print {
    pub output: Box<dyn Expression>
}

// impl Statement for Binding {
//     fn eval(&self, environment: &mut Environment) -> Option<Object> {
//         let entry = environment.scope.entry();
//     }
// }

pub struct Block {
    pub statements: Vec<Box<dyn Statement>>,
}