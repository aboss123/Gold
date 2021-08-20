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
  Function(/* name */ String, /* Stmts */ Vec<Expr>, /* Ret */ Box<Expr>),

  Else(Vec<Expr>),
  Elif(Box<Expr>, Vec<Expr>),
  If(Box<Expr>, Vec<Expr>, Option<Vec<Expr>>, Option<Box<Expr>>),

  // is
  Equality(Box<Expr>, Box<Expr>),

  // !is
  NotEqual(Box<Expr>, Box<Expr>),

  // >
  GreaterThan(Box<Expr>, Box<Expr>),

  // <
  LessThan(Box<Expr>, Box<Expr>),

  // >=
  GreaterThanEqual(Box<Expr>, Box<Expr>),

  // <=
  LessThanEqual(Box<Expr>, Box<Expr>),

  // +
  Addition(Box<Expr>, Box<Expr>),

  // -
  Subtraction(Box<Expr>, Box<Expr>),

  // *
  Multiplication(Box<Expr>, Box<Expr>),

  // /
  Division(Box<Expr>, Box<Expr>),

  // Exponent
  Power(Box<Expr>, Box<Expr>)
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

  pub rule function() -> Expr 
      = "//" _ function_name:identifier() _ "is" _ "a" _ "function." _ 
        "//" _ "Params:" 
        params:parameters() _ 
        ret:return_stmt()
      {
          Expr::Function(function_name, params, Box::new(ret))
      }   

  pub rule parameter_decl() -> Expr 
      = _ "//" _ "'" param_name:identifier() "'" _ "is" _ "of" _ "type" _ 
      ty:identifier() "."
      {
          Expr::Parameter(Parameter {
              name: param_name,
              typename: Type::from(ty)
          })
      }

    
  pub rule expression() -> Expr
      = if_expr()
      / binary_op()

  pub rule statements() -> Vec<Expr>
      = stmt:(expression()*) { stmt }
  
  pub rule else_expr() -> Expr
      = "else" _ "{" _ body:statements() _ "}"
      {
        Expr::Else(body)
      }
  
  pub rule elif() -> Expr 
      = "elif" _ expr:expression() _ "{"
            body:statements() _
        "}"
      {
        Expr::Elif(Box::new(expr), body)
      }
  
  pub rule if_expr() -> Expr
      = "if" _ expr:expression() _ "{"  _
          if_body:statements() _ 
        "}" _ elif_body:(elif()*) _ else_body:(else_expr()?)

      {
        
        Expr::If(Box::new(expr), if_body, 
          if elif_body.len() > 0 {
            Some(elif_body)
          } else {
            None
          }, 
          match else_body {
            Some(v) => Some(Box::new(v)),
            None => None
          })
      }


  pub rule binary_op() -> Expr = precedence! {
    lhs:@ _ "is" _ rhs:(@) { Expr::Equality(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ "is" _ "not" _ rhs:(@) { Expr::NotEqual(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ "<" _ rhs:(@) { Expr::LessThan(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ ">" _ rhs:(@) { Expr::GreaterThan(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ "<=" _ rhs:(@) { Expr::GreaterThanEqual(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ ">=" _ rhs:(@) { Expr::LessThanEqual(Box::new(lhs), Box::new(rhs)) }
    --
    lhs:@ _ "+" _ rhs:(@) { Expr::Addition(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ "-" _ rhs:(@) { Expr::Subtraction(Box::new(lhs), Box::new(rhs)) }
    --
    lhs:@ _ "*" _ rhs:(@) { Expr::Multiplication(Box::new(lhs), Box::new(rhs)) }
    lhs:@ _ "/" _ rhs:(@) { Expr::Division(Box::new(lhs), Box::new(rhs)) }
    --
    lhs:@ _ "^" _ rhs:(@) { Expr::Power(Box::new(lhs), Box::new(rhs)) }
    --

    "(" _ expr:expression() _ ")" { expr }
    lit:literal() { lit }
  }

  pub rule parameters() -> Vec<Expr>
      = params:(parameter_decl()*) { params }

  pub rule return_stmt() -> Expr 
      = "//" _ "Returns:" _ ty:identifier() {
          Expr::ReturnComment(Type::from(ty))
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