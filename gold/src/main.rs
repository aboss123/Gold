use gold_frontend::frontend;
use peg::{error::ParseError, str::LineCol};
use gold_frontend::parse::{Parser};

fn main() -> Result<(), String> {
    let mut parser = Parser::new("test.gld")?;
    parser.parse_file()?;
    Ok(())
}

const INC_STR: &str = include_str!("tests/test.gld");
const FUNC_CODE: &str = "// foo is a function.\n// Params:\n// 'a' is of type Int.\n// 'b' is of type String.";