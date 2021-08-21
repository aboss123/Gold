use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

use cranelift::codegen::ir::Inst;
use gold_frontend::frontend::{Expr, Type};
use gold_frontend::error::{TypeError, report_type_error};

pub struct FuncSig {
  return_type: Type,
  param_types: Vec<(Type, Range<usize>)>,
  error_metadata: Range<usize>
}

pub struct VarSig {
  ty: Type
}

pub struct Analysis {
  functions: HashMap<String, FuncSig>,
  variables: HashMap<String, VarSig>,
  source: String,
  filename: String,
  errors: usize
}

impl Analysis {
  pub fn new(src: String, filename: String) -> Self {
    Self {
      functions: HashMap::new(),
      variables: HashMap::new(),
      source: src,
      filename: filename,
      errors: 0
    }
  }
}

pub trait Lower {
  fn get_type(&self, func_ref: &HashMap<String, FuncSig>, var_ref: &HashMap<String, VarSig>) -> Type;
  fn lower_expr(&self, typechecker: &mut Analysis);
}

impl Lower for Expr {

  fn get_type(&self, func_ref: &HashMap<String, FuncSig>, var_ref: &HashMap<String, VarSig>) -> Type {
    match self {
        Expr::NoExpr => unreachable!(),
        Expr::Number(_, _) => Type::Int,
        Expr::String(_, _) => Type::String,
        Expr::Parameter(param, _) => param.typename,
        Expr::Function(_, _, ty, _, _) => *ty,
        Expr::Else(stmts, _)         => stmts.last().unwrap().get_type(func_ref, var_ref),
        Expr::Elif(_, stmts, _)      => stmts.last().unwrap().get_type(func_ref, var_ref),
        Expr::If(_, stmts, _, _, _)  => stmts.last().unwrap().get_type(func_ref, var_ref),
        Expr::While(_, stmts, _)     => stmts.last().unwrap().get_type(func_ref, var_ref),
        Expr::Call(name, _, _, _) => { 
          let probably_correct_func = func_ref.get(name).unwrap();
          probably_correct_func.return_type
        }
        Expr::List(values, _)  => values.first().unwrap().get_type(func_ref, var_ref),
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
        Expr::Var(ident, _)   => todo!(),
        Expr::Assign(_, e, _) => e.get_type(func_ref, var_ref),
        Expr::Reassign(_, e, _) => e.get_type(func_ref, var_ref),
    }
  }

  fn lower_expr(&self, typechecker: &mut Analysis) {
    match self {
        Expr::NoExpr => unreachable!(),
        Expr::Var(sym, err) => {
          match typechecker.variables.get(sym) {
            Some(var) => {},
            None => {
              report_type_error(TypeError::NotDefined(err.to_owned()), typechecker.filename.as_str(), typechecker.source.as_str());
            }
          }
        }
        Expr::Assign(name, expr, _) => {
          typechecker.variables.insert(name.to_owned(), VarSig { ty: expr.get_type(&typechecker.functions, &typechecker.variables) });
          expr.lower_expr(typechecker);
        }
        Expr::Reassign(name, expr, err) => {
          match typechecker.variables.get(name) {
            Some(_) => { expr.lower_expr(typechecker); },
            None => {
              report_type_error(TypeError::NotDefined(err.to_owned()), typechecker.filename.as_str(), typechecker.source.as_str());
            }
          }
        }
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
        Expr::If(cond, body, elifs, else_body, loc) => {
          cond.lower_expr(typechecker);
          for stmt in body { stmt.lower_expr(typechecker); }
          if elifs.is_some() {
            for stmt in elifs.as_ref().unwrap() { stmt.lower_expr(typechecker); }
          }
          if else_body.is_some() {
            for stmt in elifs.as_ref().unwrap() { stmt.lower_expr(typechecker); }
          }
        }
        Expr::Call(name, args, nloc, arg_loc) => {
          let probably_correct_func = (&typechecker).functions.get(name);
          match probably_correct_func {
            Some(func) => {
              if args.len() != func.param_types.len() {
                report_type_error(TypeError::IncorrectNumberOfFunctionArguments(arg_loc.to_owned(), func.param_types.len(), args.len()), typechecker.filename.as_str(), typechecker.source.as_str());
              }
              for (pos, arg) in args.iter().enumerate() {
                let (ty, def) = func.param_types.get(pos).unwrap();
                let arg_type = arg.get_type(&typechecker.functions, &typechecker.variables);
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
        Expr::While(cond, body, _) =>  {
          cond.lower_expr(typechecker);
          for stmt in body { stmt.lower_expr(typechecker); }
        }
        Expr::List(_, _) => todo!(),
        Expr::Equality(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::NotEqual(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::GreaterThan(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::LessThan(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::GreaterThanEqual(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::LessThanEqual(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::Addition(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::Subtraction(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::Multiplication(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::Division(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
        Expr::Power(lhs, rhs) => {
          let (lhs_ty, rhs_ty) = (lhs.get_type(&typechecker.functions, &typechecker.variables), rhs.get_type(&typechecker.functions, &typechecker.variables));
          if lhs_ty != rhs_ty {
            report_type_error(TypeError::InvaidTypesForOperation(lhs.expression_range(), rhs.expression_range(), lhs_ty, rhs_ty), typechecker.filename.as_str(), typechecker.source.as_str());
          }
        }
    }
  }
}