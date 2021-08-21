use crate::mir::types::Type;

pub enum Statement {
    Declaration(Declaration),
}

pub struct Declaration {
    pub ident: String,
    pub type_sig: Type,
}