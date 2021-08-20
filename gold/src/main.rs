use gold_frontend::frontend;

fn main() {
    gold_frontend::frontend::parser::function(INC_STR).unwrap();
}

const INC_STR: &str = include_str!("tests/test.gld");
const FUNC_CODE: &str = "// foo is a function.\n// Params:\n// 'a' is of type Int.\n// 'b' is of type String.";