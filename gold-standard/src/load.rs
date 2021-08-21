use std::mem::transmute;

use cranelift_jit::JITBuilder;

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