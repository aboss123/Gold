use std::collections::HashMap;

use cranelift::codegen::Context;
use cranelift::prelude::{AbiParam, types::*};
use cranelift::prelude::{InstBuilder, Value};
use cranelift::{codegen::ir, prelude::IntCC, prelude::FloatCC};
use cranelift::frontend::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, FuncId, Linkage, Module};
use gold_frontend::frontend::{Expr, Type};

pub enum InstructionValue {
  IntegerConstant(u64),
  FloatConstant(f64),
  StringConstant(String),
}

pub enum Instruction {
  Null,
  Push(InstructionValue),
  Add(InstructionValue, InstructionValue),
  Sub(InstructionValue, InstructionValue),
  Div(InstructionValue, InstructionValue),
  Mul(InstructionValue, InstructionValue),
  Cmp(IntCC, InstructionValue, InstructionValue),
  FCmp(FloatCC, InstructionValue, InstructionValue),
  Assign(String, InstructionValue),
  While(Box<Instruction>, Vec<Instruction>)
}

pub struct Function {
  name: String,
  params: Vec<AbiParam>,
  function_body: Vec<Instruction>,
}

pub struct Program {
  global_data: Vec<String>,
  functions  : Vec<Function>,
  instrinsics: Vec<FuncId>,
}