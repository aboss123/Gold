// use std::collections::HashMap;
//
// pub enum AlgebraicMode {
//     Sum,
//     Product
// }


#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Hash)]
pub enum ScalarType {
    Char,
    Int,
    Float,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Hash)]
pub enum Type {
    Void,
    Scalar(ScalarType),
    Array(Box<Type>),
    // Composite(CompositeType)
}



impl Type {
    pub fn as_scalar(&self) -> Option<&ScalarType> {
        if let Self::Scalar(scalar) = self {
            Some(scalar)
        } else {
            None
        }
    }
}

//
// pub struct CompositeType {
//     pub mode: AlgebraicMode,
//     pub fields: HashMap<String, Type>
// }