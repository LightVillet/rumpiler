mod tokenizer;
mod ast;
mod codegen;

use crate::compiler::tokenizer::tokenize;
use crate::compiler::ast::parse_program;
use crate::compiler::codegen::generate_asm;

pub fn compile(code: String) -> String {
    let tokenized = tokenize(code);
    let ast = parse_program(tokenized).unwrap();
    let asm = generate_asm(ast).unwrap();
    return asm;
}