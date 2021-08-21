use core::ops::Range;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Type {
  Int,
  Float,
  Number,
  String,
  Bool,
  Void
}

pub struct Parameter {
  pub name: String,
  pub typename: Type
}

pub enum Expr {
  NoExpr,
  Number(u64, Range<usize>),
  String(String, Range<usize>),
  Parameter(Parameter, Range<usize>),
  Function(/* name */ String, /* params */ Vec<(Parameter, Range<usize>)>, /* Ret */ Type, /* stmts */ Vec<Expr>, Range<usize>),

  Else(Vec<Expr>, Range<usize>),
  Elif(Box<Expr>, Vec<Expr>, Range<usize>),
  If(Box<Expr>, Vec<Expr>, Option<Vec<Expr>>, Option<Box<Expr>>, Range<usize>),

  Call(String, Vec<Expr>, Range<usize>, Range<usize>),

  While(Box<Expr>, Vec<Expr>, Range<usize>),
  List(Vec<Expr>, Range<usize>),

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
          "Int"     => Type::Int,
          "String"  => Type::String,
          "Void"    => Type::Void,
          "Bool"    => Type::Bool,
          _ => Type::Int
      }
  }
}


impl Type {

  pub fn as_str(&mut self) -> &str {
    match self {
        Type::Int => "Int",
        Type::Float => "Float",
        Type::Number => "Number",
        Type::String => "String",
        Type::Bool => "Bool",
        Type::Void => "Void",
    }
  }
}


peg::parser!(pub grammar parser() for str {


  pub rule function() -> Expr 
      = "//" start:position!() _ function_name:identifier() end:position!() _ "is" _ "a" _ "function." _ 
        "//" _ "Params:" 
        params:parameters() _ 
        ret:return_stmt() _ 
        "fn" _ "{" _ stmts:statements() _ "}"
      {
          Expr::Function(function_name, params, ret, stmts, start..end)
      }   

  pub rule parameter_decl() -> (Parameter, Range<usize>)
      = _ "//" _  "'" param_name:identifier() "'" _ "is" _ "of" _ "type" _ 
      start:position!() ty:identifier() end:position!() "."
      {
          (Parameter {
              name: param_name,
              typename: Type::from(ty)
          }, start..end)
      }


  pub rule list() -> Expr 
      = start:position!() "[" _ values:((_ expr:expression() _ {expr}) ** ",") _ "]" end:position!()
      {Expr::List(values, start..end)}
    
  pub rule expression() -> Expr
      = if_expr()
      / while_expr()
      / binary_op()

  pub rule statements() -> Vec<Expr>
      = stmt:(expression()*) { stmt }
  
  pub rule else_expr() -> Expr
      = start:position!() "else" _ "{" _ body:statements() _ "}" end:position!()
      {
        Expr::Else(body, start..end)
      }
  
  pub rule elif() -> Expr 
      = start:position!() "elif" _ expr:binary_op() _ "{"
            body:statements() _
        "}" end:position!()
      {
        Expr::Elif(Box::new(expr), body, start..end)
      }
  
  pub rule if_expr() -> Expr
      = start:position!() "if" _ expr:binary_op() _ "{"  _
          if_body:statements() _ 
        "}" end:position!() _ elif_body:(elif()*) _ else_body:(else_expr()?) 

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
          }, start..end)
      }

  pub rule while_expr() -> Expr 
      = start:position!() "while" _ cond:binary_op() _ "{"  _ 
          stmts:statements()
      "}" end:position!() 
      {
        Expr::While(Box::new(cond), stmts, start..end)
      }

  pub rule binary_op() -> Expr = precedence! {
    lhs:@ _ "is" _ start:position!() rhs:(@) { Expr::Equality(Box::new(lhs), Box::new(rhs)) }
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

    start:position!() func_name:identifier() end:position!() _ "(" s2:position!() values:((_ expr:expression() _ {expr}) ** ",") e2:position!() _ ")"
    { Expr::Call(func_name, values, start..end, s2..e2) }

    "(" _ expr:expression() _ ")" { expr }
    lit:literal() { lit }
    li:list() { li }
  }

  pub rule parameters() -> Vec<(Parameter, Range<usize>)>
      = params:(parameter_decl()*) { params }

  pub rule return_stmt() -> Type 
      = "//" _ "Returns:" _ ty:identifier() {
          Type::from(ty)
      }

  #[cache]
  pub rule identifier() -> String 
    = ident:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '_' | '0'..='9']*) 
    { ident.to_owned() }

  
  pub rule literal() -> Expr 
    = start:position!() number:$(['0'..='9']+) end:position!() {
      Expr::Number(number.parse().unwrap(), start..end)
    }
    / start:position!() "\"" s:$([^'"'..='"']+) "\"" end:position!() {
      Expr::String(s.to_owned(), start..end)
    }
    

  // Ignore these rules
  #[cache]
  rule _() = quiet!{[' ' | '\t' | '\n' | '\r']*}
});