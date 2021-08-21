use std::mem::transmute;

use cranelift_jit::JITBuilder;

const SYMBOLS: [(&str, *const u8); 4] = [
    ("print", unsafe { transmute(&crate::io::print) }),
    ("println", unsafe { transmute(&crate::io::println) }),
    ("malloc", unsafe { transmute(&crate::mem::malloc) }),
    ("free", unsafe { transmute(&crate::mem::free) }),
];

pub fn load_symbols(jit_builder: &mut JITBuilder) {
    jit_builder.symbols(SYMBOLS);
}