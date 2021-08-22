use std::{any::Any, collections::HashMap, ops::Range};

use cranelift::{codegen::{Context, ir::function}, frontend::{FunctionBuilder, FunctionBuilderContext, Variable}, prelude::{AbiParam, Block, EntityRef, InstBuilder, Signature, Value, types}};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use gold_frontend::frontend::{Parameter, Expr, Type};
use gold_standard::{load_symbols, declare_functions};
use crate::analysis::lower::{Analyzer, Lower, VarSig};

pub struct Compilation {
    variables: HashMap<String, Variable>,
    variable_index: usize,

    uncompiled_functions: Vec<Expr>,
    syntax_analyzer: Analyzer
}

impl Compilation {
    pub fn new(analyzer: Analyzer, functions: Vec<Expr>) -> Self {
        Self {
            variables: HashMap::new(),
            variable_index: 0,
            uncompiled_functions: functions,
            syntax_analyzer: analyzer
        }
    }

    fn gen_expr(&mut self, 
                scope_index: &usize, 
                expr: &Expr, 
                module: &mut JITModule, 
                builder: &mut FunctionBuilder) -> Value {
        match expr {
            Expr::Number(lit, _) => {
                builder.ins().iconst(types::I64, *lit)
            }
            Expr::String(lit, _) => todo!(),
            Expr::Var(name, _) => {
                let var_sig = self.syntax_analyzer.variables.get(*scope_index, name.to_owned());
                let var = self.variables.entry(name.to_string()).or_insert( {
                    let var = Variable::new(self.variable_index);
                    builder.declare_var(var, var_sig.ty.into());
                    var
                });
                builder.use_var(*var)
            }
            Expr::Assign(name, value, _) => {
                let val = self.gen_expr(scope_index, value, module, builder);
                let var = self.variables.get(name).unwrap();
        
                builder.def_var(*var, val);
                val
            }
            Expr::Reassign(name, value, _) => {
                let val = self.gen_expr(scope_index, value, module, builder);
                let var = self.variables.get(name).unwrap();
                builder.def_var(*var, val);
                val
            }
            Expr::Block(stmts, _) => {
                for pos in 0..stmts.len()-1 {
                    let stmt = stmts.get(pos).unwrap();
                    self.gen_expr(scope_index, stmt, module, builder);
                }
                self.gen_expr(scope_index, stmts.get(stmts.len()-1).unwrap(), module, builder)
            }
            Expr::Else(_, _) => todo!(),
            Expr::Elif(_, _, _) => todo!(),
            Expr::If(_, _, _, _, _) => todo!(),
            Expr::Call(_, _, _, _) => todo!(),
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
            _ => todo!()
        }
    }

    fn gen_fn(&mut self, function: Expr) {
        let mut builder = JITBuilder::new(cranelift_module::default_libcall_names());
    
        // Declare all compiler builtins
        load_symbols(&mut builder);
    
        let mut module = JITModule::new(builder);
        let instrinsics = declare_functions(&mut module);
    
        let mut codegen_ctx = module.make_context();
        let mut function_ctx = FunctionBuilderContext::new();
   
        
        match function {
            Expr::Function(name, params, ty, mut function_body, _) => {

                // Cranelift Function Signature
                let mut fn_signature = module.make_signature();
                fn_signature.returns.push(ty.into());

                // Setup variable parameters
                let parameters = params.iter().map(|p| {
                    let var = Variable::new(self.variable_index);
                    self.variables.insert(p.0.name.to_owned(), var);
                    self.variable_index += 1;
                    fn_signature.params.push(p.0.typename.into());
                    var
                }).collect::<Vec<Variable>>();
                


                let function_id = module
                    .declare_function(name.as_str(), Linkage::Import, &fn_signature)
                    .unwrap();

                codegen_ctx.func.signature = fn_signature;

                //===================== Code generation =====================

                // Setup the function builder
                let mut function_builder= FunctionBuilder::new(&mut codegen_ctx.func, &mut function_ctx);

                // Create entry block into the function and paramters
                let entry = function_builder.create_block();
                function_builder.append_block_params_for_function_params(entry);

                // Initialize parameters for the entry block
                for pos in 0..params.len() {
                    let val = function_builder.block_params(entry)[pos];
                    let var = parameters.get(pos).unwrap();
                    function_builder.def_var(*var, val);
                }


                //========================================================================================
                let mut define_variable = |ty, name: &String| {
                    let var = Variable::new(self.variable_index);
                    if let None = self.variables.get(name) {
                        self.variables.insert(name.to_owned(), var);
                        self.variable_index += 1;
                        function_builder.declare_var(var, ty);
                    }
                    var
                };
                //========================================================================================
                
                let zero = function_builder.ins().iconst(types::I32, 0);
                let return_variable = define_variable(types::I32, &"why".to_owned());
                function_builder.def_var(return_variable, zero);

                // Start codegen at the entry block and seal it to tell Cranelift 
                // that we have no blocks previous to this one.
                function_builder.switch_to_block(entry);
                function_builder.seal_block(entry);
    
                // Generate Cranelift IR for function body
                let variable_index = self.syntax_analyzer.functions.get(&name).unwrap().scope_index;
                match function_body.as_mut() {
                    Expr::Block(stmts, _) => {
                        for stmt in stmts { 
                            self.gen_expr(&variable_index, stmt, &mut module, &mut function_builder); 
                        }
                    }
                    _ => unreachable!()
                }

                let return_var = self.variables.get(&"why".to_owned()).unwrap();
                let return_val = function_builder.use_var(*return_var);

                function_builder.ins().return_(&[return_val]);
                function_builder.finalize();

                module.define_function(function_id,&mut codegen_ctx, &mut codegen::binemit::NullTrapSink {});

                module.clear_context(&mut codegen_ctx);
                module.finalize_definitions();

                let code = module.get_finalized_function(function_id);
            }
            _ => unreachable!()
        };
    }


}