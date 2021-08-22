use std::collections::HashMap;
use std::mem::transmute;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use target_lexicon::Triple;

use crate::io::{print, println};
use crate::math::ipowi;
use crate::mem::{free, malloc};

const PRINT_SYMBOL: &str = "print";
const PRINT_ADDRESS: *const u8 = unsafe { transmute(print as unsafe extern "C" fn(_) -> _) };

const PRINTLN_SYMBOL: &str = "println";
const PRINTLN_ADDRESS: *const u8 = unsafe { transmute(println as unsafe extern "C" fn(_) -> _) };

const MALLOC_SYMBOL: &str = "malloc";
const MALLOC_ADDRESS: *const u8 = unsafe { transmute(malloc as unsafe extern "C" fn(_) -> _) };

const FREE_SYMBOL: &str = "free";
const FREE_ADDRESS: *const u8 = unsafe { transmute(free as unsafe extern "C" fn(_)) };

const IPOWI_SYMBOL: &str = "ipowi";
const IPOWI_ADDRESS: *const u8 = unsafe { transmute(ipowi as unsafe extern "C" fn(_, _) -> _) };


const SYMBOLS: [(&str, *const u8); 5] = [
    (PRINT_SYMBOL, PRINT_ADDRESS),
    (PRINTLN_SYMBOL, PRINTLN_ADDRESS),
    (MALLOC_SYMBOL, MALLOC_ADDRESS),
    (FREE_SYMBOL, FREE_ADDRESS),
    (IPOWI_SYMBOL, IPOWI_ADDRESS)
];

pub fn load_symbols(jit_builder: &mut JITBuilder) {
    jit_builder.symbols(SYMBOLS);
}

fn fn_declare<'a>(
    module: &mut JITModule,
    ids: &mut HashMap<&'a str, FuncId>,
    name: &'a str,
    params: &[AbiParam],
    out_param: Option<&AbiParam>,
) {
    let mut sig = module.make_signature();

    for p in params.iter() {
        sig.params.push(p.to_owned());
    }

    out_param.into_iter().for_each(|p| {
        sig.returns.push(p.to_owned());
    });

    let fid = module.declare_function(name, Linkage::Import, &sig).unwrap();

    ids.insert(name, fid);
}

pub fn declare_functions(module: &mut JITModule) -> HashMap<&'static str, FuncId> {
    let ptr_type = AbiParam::new(Type::triple_pointer_type(&Triple::host()));
    let int_type = AbiParam::new(types::I64);

    let mut ids = HashMap::new();

    fn_declare(module, &mut ids, PRINT_SYMBOL, &[ptr_type.clone()], Some(&int_type));
    fn_declare(module, &mut ids, PRINTLN_SYMBOL, &[ptr_type.clone()], Some(&int_type));
    fn_declare(module, &mut ids, MALLOC_SYMBOL, &[int_type.clone()], Some(&ptr_type));
    fn_declare(module, &mut ids, FREE_SYMBOL, &[ptr_type.clone()], None);
    fn_declare(module, &mut ids, IPOWI_SYMBOL, &[int_type.clone(), int_type.clone()], Some(&int_type));

    ids
}