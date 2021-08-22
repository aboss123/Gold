use peg::{error::ParseError, str::LineCol};

use gold_backend::{analysis::lower::Lower, lir};
use gold_backend::analysis::lower::Analyzer;
use gold_frontend::frontend;
use gold_frontend::parse::Parser;
use gold_backend::codegen::Compilation;
use std::env::args;

fn main() -> Result<(), String> {
    let file = args().skip(1).next().expect("Need a file path please!");
    let mut parser = Parser::new(&file)?;
    let expr = parser.parse_file()?;
    let mut analyze = Analyzer::new(parser.file_contents, parser.filename.to_owned());
    expr.typecheck(&mut analyze);

    let mut comp = Compilation::new(analyze, vec![expr.clone()]);

    comp.gen_fn(expr);

    Ok(())
}