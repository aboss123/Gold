use crate::mir::expressions::{Expression, Environment};
use crate::mir::types::Type;
use crate::mir::expressions::literal::Object;

pub struct VariableExpression {
    pub name: String,
}

impl Expression for VariableExpression {
    fn get_type(&self, environment: &Environment) -> Type {
        environment.scope[&self.name].get_type(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        environment.scope[&self.name].eval(environment)
    }
}