use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{SimpleFiles, Error};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use peg::error::ParseError;
use peg::str::LineCol;

pub enum TypeError {
  // TODO: Make this something useful
}

pub fn report_parse_error(filename: &str, source: &str, err: ParseError<LineCol>) -> Result<(), Error> {
  let mut file_handler = SimpleFiles::new();

  let file_id = file_handler.add(filename, source);
  let err = Diagnostic::error()
      .with_message("Something")
      .with_labels(vec! [
        Label::primary(file_id, 0..err.location.offset)
      ]);
  let writer = StandardStream::stderr(ColorChoice::Always);
  let config = codespan_reporting::term::Config::default();
  codespan_reporting::term::emit(&mut writer.lock(), &config, &file_handler, &err)?;
  Ok(())
}