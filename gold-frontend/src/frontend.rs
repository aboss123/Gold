pub enum Type {
  Int,
  String,
}

pub struct Parameter {
  pub name: String,
  pub typename: Type
}

pub enum Expr {
  Number(u64),
  String(String),
  Parameter(Parameter),
  ReturnComment(Type),
  Function(/* name */ String, /* Stmts */ Vec<Expr>, /* Ret */ Box<Expr>)
}

impl From<String> for Type {

  fn from(input: String) -> Type {
      match input.as_str() {
          "Int" => Type::Int,
          "String" => Type::String,
          _ => Type::Int
      }
  }
}


peg::parser!(pub grammar parser() for str {

  pub rule parameter_decl() -> Expr 
      = _ "//" _ "'" param_name:identifier() "'" _ "is" _ "of" _ "type" _ 
      ty:identifier() "."
      {
          Expr::Parameter(Parameter {
              name: param_name,
              typename: Type::from(ty)
          })
      }

  pub rule parameters() -> Vec<Expr>
      = params:(parameter_decl()*) { params }

  pub rule return_stmt() -> Expr 
      = "//" _ "Returns:" _ ty:identifier() {
          Expr::ReturnComment(Type::from(ty))
      }
 
  pub rule function() -> Expr 
      = "//" _ function_name:identifier() _ "is" _ "a" _ "function." _ 
        "//" _ "Params:" 
        params:parameters() _ 
        ret:return_stmt()
      {
          Expr::Function(function_name, params, Box::new(ret))
      }   

  pub rule identifier() -> String 
    = ident:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '_' | '0'..='9']*) 
    { ident.to_owned() }

  pub rule literal() -> Expr 
    = number:$(['0'..='9']+) {
      Expr::Number(number.parse().unwrap())
    }
    / "\"" s:$([^'"'..='"']+) "\"" {
      Expr::String(s.to_owned())
    }
    

  // Ignore these rules
  rule _() = quiet!{[' ' | '\t' | '\n' | '\r']+}
});