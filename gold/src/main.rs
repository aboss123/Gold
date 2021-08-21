use gold_frontend::frontend;
use gold_backend::{analysis::lower::Lower, lir};
use peg::{error::ParseError, str::LineCol};
use gold_frontend::parse::{Parser};
use gold_backend::analysis::lower::Analysis;

fn main() -> Result<(), String> {
    let mut parser = Parser::new("tests/another.gold")?;
    let expr = parser.parse_file()?;
    let mut analyze = Analysis::new(parser.file_contents, parser.filename.to_owned());
    expr.lower_expr(&mut analyze);

    Ok(())
}