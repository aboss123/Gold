use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{SimpleFiles, Error};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use peg::error::ParseError;
use peg::str::LineCol;

use crate::frontend::{Expr, Type};

impl Expr {

  pub fn expression_range(&self) -> Range<usize> {
    match self {
        Expr::NoExpr => unreachable!(),
        Expr::Number(_, r) => r.to_owned(),
        Expr::String(_, r) => r.to_owned(),
        Expr::Parameter(_, r) => r.to_owned(),
        Expr::Function(_, _, _, _, r) => r.to_owned(),
        Expr::Else(_, r) => r.to_owned(),
        Expr::Elif(_, _, r) => r.to_owned(),
        Expr::If(_, _, _, _, r) => r.to_owned(),
        Expr::Call(_, _, r, _) => r.to_owned(),
        Expr::While(_, _, r) => r.to_owned(),
        Expr::List(_, r) => r.to_owned(),
        Expr::Equality(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::NotEqual(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::GreaterThan(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::LessThan(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::GreaterThanEqual(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::LessThanEqual(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::Addition(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::Subtraction(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::Multiplication(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::Division(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
        Expr::Power(lhs, rhs) => lhs.expression_range().start..rhs.expression_range().end,
    }
  }
}

pub enum TypeError {
  // TODO: Make this something useful
  InvaidTypesForOperation(Range<usize>, Range<usize>),
  NotEqualFunctionReturnType(Range<usize>, Range<usize>),
  FunctionDoesNotExist(String, Range<usize>),
  IncorrectNumberOfFunctionArguments(Range<usize>, usize, usize),
  IncorrectTypeValueForArgument(Range<usize>, Range<usize>, Type, Type)
}

pub fn report_type_error(error: TypeError, filename: &str, source: &str) -> Result<(), Error> {
  let mut file_handler = SimpleFiles::new();
  let file_id = file_handler.add(filename, source);

  let err: Diagnostic<usize>;
  match error {
    TypeError::InvaidTypesForOperation(s1, s2) => todo!(),
    TypeError::NotEqualFunctionReturnType(s1, s2) => todo!(),
    TypeError::FunctionDoesNotExist(name, loc) =>  {
      err = Diagnostic::error()
        .with_message(["Function with the name '", name.as_str(), "'", " does not exist"].join(""))
        .with_labels(vec![
          Label::primary(file_id, loc)
        ]);
    }
    TypeError::IncorrectNumberOfFunctionArguments(s1, expected_args, got) => {
      err = Diagnostic::error()
        .with_message("Function call has incorrect number of arguments")
        .with_labels(vec![
          //Label::primary(file_id, s1),
          Label::secondary(file_id, s1).with_message(["Expected ", expected_args.to_string().as_str(), " arguments"].join(""))
        ])
        .with_notes(vec![
          ["expected ", expected_args.to_string().as_str(), " arguments but got ", got.to_string().as_str()].join("")
        ])
    }
    TypeError::IncorrectTypeValueForArgument(defined, error, mut expected, mut got) => {
      err = Diagnostic::error()
        .with_message("Incorrect type for argument")
        .with_labels(vec![
          //Label::primary(file_id, s1),
          Label::secondary(file_id, defined).with_message("Type defined here"),
          Label::primary(file_id, error).with_message(["Expected type `", expected.as_str(), "` but got type `", got.as_str(), "`"].join(""))
        ]);
    }
  }
  let writer = StandardStream::stderr(ColorChoice::Always);
  let config = codespan_reporting::term::Config::default();
  codespan_reporting::term::emit(&mut writer.lock(), &config, &file_handler, &err)?;
  Ok(())
}

pub fn report_parse_error(filename: &str, source: &str, err: ParseError<LineCol>) -> Result<(), Error> {
  let mut file_handler = SimpleFiles::new();

  let file_id = file_handler.add(filename, source);
  let mut expected = "Expected ".to_string();
  expected.push_str(err.expected.to_string().as_str());
  let err = Diagnostic::error()
      .with_message(expected.to_string())
      .with_labels(vec! [
        Label::primary(file_id, err.location.offset..err.location.offset)
      ]);
  let writer = StandardStream::stderr(ColorChoice::Always);
  let config = codespan_reporting::term::Config::default();
  codespan_reporting::term::emit(&mut writer.lock(), &config, &file_handler, &err)?;
  Ok(())
}