use std::ops::Range;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{SimpleFiles, Error};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use peg::error::ParseError;
use peg::str::LineCol;

pub enum TypeError {
  // TODO: Make this something useful
  InvalidBinaryOperation(Range<usize>, Range<usize>),
  NotEqualFunctionReturnType(Range<usize>, Range<usize>),
}

pub fn report_type_error(error: TypeError, source: &str) -> Result<(), Error> {
  match error {
    TypeError::InvalidBinaryOperation(s1, s2) => todo!(),
    TypeError::NotEqualFunctionReturnType(s1, s2) => todo!(),
}
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