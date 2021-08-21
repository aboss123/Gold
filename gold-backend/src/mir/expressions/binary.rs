use crate::mir::expressions::{Environment, Expression};
use crate::mir::expressions::literal::{Object, ScalarObject};
use crate::mir::types::Type;

pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub lhs: Box<dyn Expression>,
    pub rhs: Box<dyn Expression>,
}

impl Expression for BinaryExpression {
    fn get_type(&self, environment: &Environment) -> Type {
        let left = self.lhs.get_type(environment);
        let right = self.rhs.get_type(environment);

        assert_eq!(left, right);

        left
    }

    fn eval(&self, environment: &Environment) -> Object {
        // type check
        let _ = self.get_type(environment);

        let left = self.lhs.eval(environment);
        let right = self.rhs.eval(environment);

        match left {
            Object::Scalar(ScalarObject::String(l)) => {
                if let Object::Scalar(ScalarObject::String(r)) = right {
                    Object::Scalar(ScalarObject::String(l + &r))
                } else {
                    unreachable!();
                }
            }
            Object::Scalar(ScalarObject::Int(l)) => {
                if let Object::Scalar(ScalarObject::Int(r)) = right {
                    Object::Scalar(ScalarObject::Int(l + r))
                } else {
                    unreachable!();
                }
            }
            Object::Scalar(ScalarObject::Float(l)) => {
                if let Object::Scalar(ScalarObject::Float(r)) = right {
                    Object::Scalar(ScalarObject::Float(l + r))
                } else {
                    unreachable!();
                }
            }
        }
    }
}

pub enum BinaryOperator {
    Add,
    Sub,
    Div,
    Mul,
    LessThan,
    GreaterThan,
}