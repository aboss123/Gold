use cranelift::codegen::ir::Inst;
use gold_frontend::frontend::{Expr, Type};
use gold_frontend::error::TypeError;
use crate::lir::config::{Function, Instruction, InstructionValue};

pub trait Lower {
  fn get_type(&self) -> Type;
}

impl Lower for Expr {

  fn get_type(&self) -> Type {
    match self {
        Expr::NoExpr => unreachable!(),
        Expr::Number(_) => Type::Int,
        Expr::String(_) => Type::String,
        Expr::Parameter(param, _) => param.typename,
        Expr::Function(_, _, ty, _)   => *ty,
        Expr::Else(stmts)         => stmts.last().unwrap().get_type(),
        Expr::Elif(_, stmts)      => stmts.last().unwrap().get_type(),
        Expr::If(_, stmts, _, _)  => stmts.last().unwrap().get_type(),
        Expr::While(_, stmts)     => stmts.last().unwrap().get_type(),
        Expr::Call(_, _)              => Type::Int,
        Expr::List(values)  => values.first().unwrap().get_type(),
        Expr::Equality(_, _)          => Type::Bool,
        Expr::NotEqual(_, _)          => Type::Bool,
        Expr::GreaterThan(_, _)       => Type::Bool,
        Expr::LessThan(_, _)          => Type::Bool,
        Expr::GreaterThanEqual(_, _)  => Type::Bool,
        Expr::LessThanEqual(_, _)     => Type::Bool,
        Expr::Addition(_, _)          => Type::Number,
        Expr::Subtraction(_, _)       => Type::Number,
        Expr::Multiplication(_, _)    => Type::Number,
        Expr::Division(_, _)          => Type::Number,
        Expr::Power(_, _)             => Type::Number,
    }
  }

}