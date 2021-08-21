use gold_frontend::frontend;
use gold_backend::lir;
use peg::{error::ParseError, str::LineCol};
use gold_frontend::parse::{Parser};

fn main() -> Result<(), String> {
    let mut parser = Parser::new("tests/test.gld")?;
    parser.parse_file()?;
    Ok(())
}