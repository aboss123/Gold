use peg::{error::ParseError, str::LineCol};

use gold_backend::{analysis::lower::Lower, lir};
use gold_backend::analysis::lower::Analyzer;
use gold_frontend::frontend;
use gold_frontend::parse::Parser;

fn main() -> Result<(), String> {
    let mut parser = Parser::new("tests/another.gold")?;
    let expr = parser.parse_file()?;
    let mut analyze = Analyzer::new(parser.file_contents, parser.filename.to_owned());
    expr.typecheck(&mut analyze);

    Ok(())
}