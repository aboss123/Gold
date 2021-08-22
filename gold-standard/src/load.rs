use std::collections::HashMap;
use std::mem::transmute;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};
use target_lexicon::Triple;

use crate::io::{print, println};
use crate::mem::{free, malloc};

const SYMBOLS: [(&str, *const u8); 4] = [
    ("print", unsafe { transmute(print as unsafe extern "C" fn(_) -> _) }),
    ("println", unsafe { transmute(println as unsafe extern "C" fn(_) -> _) }),
    ("malloc", unsafe { transmute(malloc as unsafe extern "C" fn(_) -> _) }),
    ("free", unsafe { transmute(free as unsafe extern "C" fn(_)) }),
];

pub fn load_symbols(jit_builder: &mut JITBuilder) {
    jit_builder.symbols(SYMBOLS);
}

pub fn declare_functions(module: &mut JITModule) -> HashMap<&'static str, FuncId> {
    let ptr_type = Type::triple_pointer_type(&Triple::host());

    let mut print_sig = module.make_signature();
    print_sig.params.push(AbiParam::new(ptr_type.clone()));
    print_sig.returns.push(AbiParam::new(types::I64));

    let mut malloc_sig = module.make_signature();
    malloc_sig.params.push(AbiParam::new(types::I64));
    malloc_sig.returns.push(AbiParam::new(ptr_type.clone()));

    let mut free_sig = module.make_signature();
    free_sig.params.push(AbiParam::new(ptr_type.clone()));

    let print_func = module.declare_function("print", Linkage::Import, &print_sig).unwrap();
    let println_func = module.declare_function("println", Linkage::Import, &print_sig).unwrap();
    let malloc_func = module.declare_function("malloc", Linkage::Import, &malloc_sig).unwrap();
    let free_func = module.declare_function("free", Linkage::Import, &free_sig).unwrap();

    let mut ids = HashMap::new();

    ids.insert("print", print_func);
    ids.insert("println", println_func);
    ids.insert("malloc", malloc_func);
    ids.insert("free", free_func);

    ids
}