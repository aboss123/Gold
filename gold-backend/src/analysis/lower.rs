use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

use cranelift::codegen::ir::Inst;
use gold_frontend::frontend::{Expr, Type};
use gold_frontend::error::{TypeError, report_parse_error, report_type_error};
use crate::lir::config::{Function, Instruction, InstructionValue};


pub struct FuncSig {
  return_type: Type,
  param_types: Vec<(Type, Range<usize>)>,
  error_metadata: Range<usize>
}

pub struct Analysis {
  functions: HashMap<String, FuncSig>,
  source: String,
  filename: String,
  errors: usize
}

impl Analysis {
  pub fn new(src: String, filename: String) -> Self {
    Self {
      functions: HashMap::new(),
      source: src,
      filename: filename,
      errors: 0
    }
  }
}

pub trait Lower {
  fn get_type(&self, func_ref: &HashMap<String, FuncSig>) -> Type;
  fn lower_expr(&self, typechecker: &mut Analysis);
}

impl Lower for Expr {

  fn get_type(&self, func_ref: &HashMap<String, FuncSig>) -> Type {
    match self {
        Expr::NoExpr => unreachable!(),
        Expr::Number(_, _) => Type::Int,
        Expr::String(_, _) => Type::String,
        Expr::Parameter(param, err) => param.typename,
        Expr::Function(_, _, ty, _, _) => *ty,
        Expr::Else(stmts, _)         => stmts.last().unwrap().get_type(func_ref),
        Expr::Elif(_, stmts, _)      => stmts.last().unwrap().get_type(func_ref),
        Expr::If(_, stmts, _, _, _)  => stmts.last().unwrap().get_type(func_ref),
        Expr::While(_, stmts, _)     => stmts.last().unwrap().get_type(func_ref),
        Expr::Call(name, _, _, _) => { 
          let probably_correct_func = func_ref.get(name).unwrap();
          probably_correct_func.return_type
        }
        Expr::List(values, _)  => values.first().unwrap().get_type(func_ref),
        Expr::Equality(_, _)          => Type::Bool,
        Expr::NotEqual(_, _)          => Type::Bool,
        Expr::GreaterThan(_, _)       => Type::Bool,
        Expr::LessThan(_, _)          => Type::Bool,
        Expr::GreaterThanEqual(_, _)  => Type::Bool,
        Expr::LessThanEqual(_, _)     => Type::Bool,
        Expr::Addition(lhs, rhs) => {
       
          Type::Number
        }
        Expr::Subtraction(_, _)       => Type::Number,
        Expr::Multiplication(_, _)    => Type::Number,
        Expr::Division(_, _)          => Type::Number,
        Expr::Power(_, _)             => Type::Number,
    }
  }

  fn lower_expr(&self, typechecker: &mut Analysis) {
    match self {
        Expr::NoExpr => unreachable!(),
        Expr::Number(_, _) => todo!(),
        Expr::String(_, _) => todo!(),
        Expr::Parameter(_, _) => todo!(),
        Expr::Function(name, params, ty, stmts, loc) => {
          typechecker.functions.insert(name.to_owned(), FuncSig { 
            return_type: *ty, 
            param_types: params.iter().map(|p| ((*p).0.typename, (*p).1.to_owned())).collect::<Vec<(Type, Range<usize>)>>(), 
            error_metadata: loc.to_owned()
          });
          for stmt in stmts { stmt.lower_expr(typechecker); }
        }
        Expr::Else(_, _) => todo!(),
        Expr::Elif(_, _, _) => todo!(),
        Expr::If(_, _, _, _, _) => todo!(),
        Expr::Call(name, args, nloc, arg_loc) => {
          let probably_correct_func = (&typechecker).functions.get(name);
          match probably_correct_func {
            Some(func) => {
              if args.len() != func.param_types.len() {
                report_type_error(TypeError::IncorrectNumberOfFunctionArguments(arg_loc.to_owned(), func.param_types.len(), args.len()), typechecker.filename.as_str(), typechecker.source.as_str());
              }
              for (pos, arg) in args.iter().enumerate() {
                let (ty, def) = func.param_types.get(pos).unwrap();
                let arg_type = arg.get_type(&typechecker.functions);
                if arg_type != *ty {
                  report_type_error(TypeError::IncorrectTypeValueForArgument(def.to_owned(), arg.expression_range(), *ty, arg_type), typechecker.filename.as_str(), typechecker.source.as_str());
                }
              }
            }
            None => {
              report_type_error(TypeError::FunctionDoesNotExist(name.to_string(), nloc.to_owned()), typechecker.filename.as_str(), typechecker.source.as_str());
            }
          }
        }
        Expr::While(_, _, _) => todo!(),
        Expr::List(_, _) => todo!(),
        Expr::Equality(_, _) => todo!(),
        Expr::NotEqual(_, _) => todo!(),
        Expr::GreaterThan(_, _) => todo!(),
        Expr::LessThan(_, _) => todo!(),
        Expr::GreaterThanEqual(_, _) => todo!(),
        Expr::LessThanEqual(_, _) => todo!(),
        Expr::Addition(_, _) => todo!(),
        Expr::Subtraction(_, _) => todo!(),
        Expr::Multiplication(_, _) => todo!(),
        Expr::Division(_, _) => todo!(),
        Expr::Power(_, _) => todo!(),
    }
  }

}