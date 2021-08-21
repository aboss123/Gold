use std::marker::PhantomData;

use crate::mir::expressions::{Environment, Expression};
use crate::mir::expressions::literal::{Object, Scalar};
use crate::mir::types::{ScalarType, Type};

pub struct BinaryArithmeticExpression<T> {
    pub lhs: Box<dyn Expression>,
    pub rhs: Box<dyn Expression>,
    pub _op: PhantomData<T>,
}

impl<T> BinaryArithmeticExpression<T> {
    pub fn type_of(&self, environment: &Environment) -> Type {
        let left = self.lhs.get_type(environment);

        let right = self.rhs.get_type(environment);

        assert_eq!(left, right, "Type error, values must be of same type");

        left
    }
}

pub struct AddOperator;

impl Expression for BinaryArithmeticExpression<AddOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let mut lhs = self.lhs.eval(environment);
        let mut rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Char(lhs.as_scalar().unwrap().as_char().unwrap()
                        + rhs.as_scalar().unwrap().as_char().unwrap()),
                    ScalarType::Int => Scalar::Int(lhs.as_scalar().unwrap().as_integer().unwrap()
                        + rhs.as_scalar().unwrap().as_integer().unwrap()),
                    ScalarType::Float => Scalar::Float(lhs.as_scalar().unwrap().as_float().unwrap()
                        + rhs.as_scalar().unwrap().as_float().unwrap()),
                })
            }
            Type::Array(_) => {
                let left = lhs.as_array_mut().unwrap();
                let right = rhs.as_array_mut().unwrap();

                left.append(right);

                lhs
            }
        }
    }
}

pub struct SubOperator;

impl Expression for BinaryArithmeticExpression<SubOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let lhs = self.lhs.eval(environment);
        let rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Char(lhs.as_scalar().unwrap().as_char().unwrap()
                        - rhs.as_scalar().unwrap().as_char().unwrap()),
                    ScalarType::Int => Scalar::Int(lhs.as_scalar().unwrap().as_integer().unwrap()
                        - rhs.as_scalar().unwrap().as_integer().unwrap()),
                    ScalarType::Float => Scalar::Float(lhs.as_scalar().unwrap().as_float().unwrap()
                        - rhs.as_scalar().unwrap().as_float().unwrap()),
                })
            }
            Type::Array(_) => panic!("Cannot subtract arrays"),
        }
    }
}

pub struct MulOperator;

impl Expression for BinaryArithmeticExpression<MulOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let lhs = self.lhs.eval(environment);
        let rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Char(lhs.as_scalar().unwrap().as_char().unwrap()
                        * rhs.as_scalar().unwrap().as_char().unwrap()),
                    ScalarType::Int => Scalar::Int(lhs.as_scalar().unwrap().as_integer().unwrap()
                        * rhs.as_scalar().unwrap().as_integer().unwrap()),
                    ScalarType::Float => Scalar::Float(lhs.as_scalar().unwrap().as_float().unwrap()
                        * rhs.as_scalar().unwrap().as_float().unwrap()),
                })
            }
            Type::Array(_) => panic!("Cannot multiply arrays"),
        }
    }
}

pub struct DivOperator;

impl Expression for BinaryArithmeticExpression<DivOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let lhs = self.lhs.eval(environment);
        let rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Char(lhs.as_scalar().unwrap().as_char().unwrap()
                        / rhs.as_scalar().unwrap().as_char().unwrap()),
                    ScalarType::Int => Scalar::Int(lhs.as_scalar().unwrap().as_integer().unwrap()
                        / rhs.as_scalar().unwrap().as_integer().unwrap()),
                    ScalarType::Float => Scalar::Float(lhs.as_scalar().unwrap().as_float().unwrap()
                        / rhs.as_scalar().unwrap().as_float().unwrap()),
                })
            }
            Type::Array(_) => panic!("Cannot divide arrays"),
        }
    }
}

pub struct LessThanOperator;

impl Expression for BinaryArithmeticExpression<LessThanOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let lhs = self.lhs.eval(environment);
        let rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_char().unwrap()
                        < rhs.as_scalar().unwrap().as_char().unwrap())),
                    ScalarType::Int => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_integer().unwrap()
                        < rhs.as_scalar().unwrap().as_integer().unwrap())),
                    ScalarType::Float => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_float().unwrap()
                        < rhs.as_scalar().unwrap().as_float().unwrap())),
                })
            }
            Type::Array(_) => panic!("Cannot compare arrays"),
        }
    }
}

pub struct GreaterThanOperator;

impl Expression for BinaryArithmeticExpression<GreaterThanOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let lhs = self.lhs.eval(environment);
        let rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_char().unwrap()
                        > rhs.as_scalar().unwrap().as_char().unwrap())),
                    ScalarType::Int => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_integer().unwrap()
                        > rhs.as_scalar().unwrap().as_integer().unwrap())),
                    ScalarType::Float => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_float().unwrap()
                        > rhs.as_scalar().unwrap().as_float().unwrap())),
                })
            }
            Type::Array(_) => panic!("Cannot compare arrays"),
        }
    }
}

pub struct EqOperator;

impl Expression for BinaryArithmeticExpression<EqOperator> {
    fn get_type(&self, environment: &Environment) -> Type {
        self.type_of(environment)
    }

    fn eval(&self, environment: &Environment) -> Object {
        let lhs = self.lhs.eval(environment);
        let rhs = self.rhs.eval(environment);

        match self.get_type(environment) {
            Type::Void => panic!("Type error, cant use void in binary expression"),
            Type::Scalar(t) => {
                Object::Scalar(match t {
                    ScalarType::Char => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_char().unwrap()
                        == rhs.as_scalar().unwrap().as_char().unwrap())),
                    ScalarType::Int => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_integer().unwrap()
                        == rhs.as_scalar().unwrap().as_integer().unwrap())),
                    ScalarType::Float => Scalar::Int(i64::from(lhs.as_scalar().unwrap().as_float().unwrap()
                        == rhs.as_scalar().unwrap().as_float().unwrap())),
                })
            }
            Type::Array(_) => panic!("Cannot compare arrays"),
        }
    }
}

// #[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
// pub enum BinaryOperator {
//     Eq,
// }