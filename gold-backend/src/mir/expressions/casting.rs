use crate::mir::types::{Type, ScalarType};
use crate::mir::expressions::{Expression, ExpressionLevelEnvironment};
use crate::mir::expressions::literal::{Object, Scalar};

pub struct CastExpression {
    pub into: ScalarType,
    pub child: Box<dyn Expression>
}

impl Expression for CastExpression {
    fn get_type(&self, _: &ExpressionLevelEnvironment) -> Type {
        Type::Scalar(self.into.clone())
    }

    fn eval(&self, environment: &ExpressionLevelEnvironment) -> Object {
        let val = self.child.eval(environment).as_scalar().cloned().expect("Must be scalar type");

        Object::Scalar(match self.into {
            ScalarType::Char => Scalar::Char(match val {
                Scalar::Char(v) => v as u8,
                Scalar::Int(v) => v as u8,
                Scalar::Float(v) => v as u8,
            }),
            ScalarType::Int => Scalar::Int(match val {
                Scalar::Char(v) => v as i64,
                Scalar::Int(v) => v as i64,
                Scalar::Float(v) => v as i64,
            }),
            ScalarType::Float => Scalar::Float(match val {
                Scalar::Char(v) => v as f64,
                Scalar::Int(v) => v as f64,
                Scalar::Float(v) => v as f64,
            }),
        })
    }
}