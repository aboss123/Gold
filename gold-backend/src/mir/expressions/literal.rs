use crate::mir::expressions::{ExpressionLevelEnvironment, Expression};
use crate::mir::types::{ScalarType, Type};

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Object {
    Void,
    Scalar(Scalar),
    Array(Vec<Object>),
}

impl Object {
    pub fn as_scalar(&self) -> Option<&Scalar> {
        if let Self::Scalar(object) = self {
            Some(object)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Object>> {
        if let Self::Array(object) = self {
            Some(object)
        } else {
            None
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Object>> {
        if let Self::Array(object) = self {
            Some(object)
        } else {
            None
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Scalar {
    Char(u8),
    Int(i64),
    Float(f64),
}

impl Scalar {
    pub fn as_integer(&self) -> Option<i64> {
        if let Self::Int(object) = self {
            Some(*object)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Self::Float(object) = self {
            Some(*object)
        } else {
            None
        }
    }

    pub fn as_char(&self) -> Option<u8> {
        if let Self::Char(object) = self {
            Some(*object)
        } else {
            None
        }
    }
}

impl Expression for Object {
    fn get_type(&self, environment: &ExpressionLevelEnvironment) -> Type {
        match self {
            Object::Scalar(s) => match s {
                Scalar::Int(_) => Type::Scalar(ScalarType::Int),
                Scalar::Float(_) => Type::Scalar(ScalarType::Float),
                Scalar::Char(_) => Type::Scalar(ScalarType::Char)
            },
            Object::Void => Type::Void,
            Object::Array(a) => {
                let t = a.first().expect("Error, cannot infer type for empty vector")
                    .get_type(environment);

                assert!(
                    a.iter().all(|x| x.get_type(environment) == t),
                    "Error,vectors must be of homogenous types"
                );

                Type::Array(Box::new(t))
            }
        }
    }

    fn eval(&self, _: &ExpressionLevelEnvironment) -> Object {
        self.clone()
    }
}