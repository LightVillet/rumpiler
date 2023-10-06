mod compiler;

use crate::compiler::tokenizer::*;
use crate::compiler::ast::*;

fn main() {
    let code = "int main() {
    return -~!-42;
}";
    println!("{}", parse_program(tokenize(code)).unwrap());
    return ();
}

