// use std::collections::HashMap;
//
// pub enum AlgebraicMode {
//     Sum,
//     Product
// }

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum ScalarType {
    Int,
    Float,
    String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Type {
    Scalar(ScalarType),
    // Array(Box<Type>),
    // Composite(CompositeType)
}

//
// pub struct CompositeType {
//     pub mode: AlgebraicMode,
//     pub fields: HashMap<String, Type>
// }