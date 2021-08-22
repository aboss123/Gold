use std::{any::Any, collections::HashMap, ops::Range};

use cranelift::{codegen::{Context, ir::function}, frontend::{FunctionBuilder, FunctionBuilderContext, Variable}, prelude::{AbiParam, Block, EntityRef, Signature, types}};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use gold_frontend::frontend::{Parameter, Expr, Type};
use gold_standard::{load_symbols, declare_functions};
use crate::analysis::lower::{Analyzer, VarSig};

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

    fn gen_expr(&mut self, module: &mut JITModule, builder: &mut FunctionBuilder) {

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
            Expr::Function(name, params, ty, stmts, _) => {

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
                

                // Start codegen at the entry block and seal it to tell Cranelift 
                // that we have no blocks previous to this one.
                function_builder.switch_to_block(entry);
                function_builder.seal_block(entry);
    
                // Declare function variables
                //let variables = 
    
            }
            _ => unreachable!()
        };
    }


}