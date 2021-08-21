use crate::mir::expressions::{Expression, Environment};
use crate::mir::types::{Type, ScalarType};

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Object {
    Scalar(ScalarObject)
}

#[derive(Clone, PartialOrd, PartialEq)]
pub enum ScalarObject {
    Int(i64),
    Float(f64),
    String(String),
}

impl Expression for Object {
    fn get_type(&self, _: &Environment) -> Type {
        match self {
            Object::Scalar(s) => {
                match s {
                    ScalarObject::Int(_) => Type::Scalar(ScalarType::Int),
                    ScalarObject::Float(_) => Type::Scalar(ScalarType::Float),
                    ScalarObject::String(_) => Type::Scalar(ScalarType::String)
                }
            }
        }
    }

    fn eval(&self, _: &Environment) -> Object {
        self.clone()
    }
}