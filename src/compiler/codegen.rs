use crate::compiler::ast::Node;
use crate::compiler::tokenizer::Token::*;
use crate::compiler::ast::ValueType::*;
use crate::compiler::ast::NonLiteralName::*;
use crate::compiler::tokenizer::UnaryOperationName::*;

pub fn generate_asm(node: Node) -> Option<String> {
    match node.value_type {
        NonLiteral(Program) => return generate_asm(node.children[0].clone()),
        NonLiteral(Function) => {
            match node.children[1].clone().value_type {
                Literal(Identifier(name)) => return Some(format!(".globl {name}\n{name}:\n{code}",
                                                                         code = (generate_asm(node.children[5].clone()).unwrap()))),
                _ => return None,
            };
        },
        NonLiteral(Statement) => {
            match node.children[1].clone().value_type {
                NonLiteral(Expression) => return Some(format!("{exp_calc}ret\n",
                                                      exp_calc = (generate_asm(node.children[1].clone()).unwrap()))),
                _ => return None,
            };
        },
        NonLiteral(Expression) =>
            match  node.children[0].clone().value_type {
                Literal(IntegerLiteral(int)) => return Some(format!("movl ${int}, %eax\n")),
                Literal(UnaryOperation(Negation)) => return Some(format!("{exp_calc}neg %eax\n",
                                                                exp_calc = (generate_asm(node.children[1].clone()).unwrap()))),
                Literal(UnaryOperation(BitwiseComp)) => return Some(format!("{exp_calc}not %eax\n",
                                                                exp_calc = (generate_asm(node.children[1].clone()).unwrap()))),
                Literal(UnaryOperation(LogNeg)) => return Some(format!("{exp_calc}cmpl $0, %eax\nmovl $0, %eax\nsete %al\n",
                                                                exp_calc = (generate_asm(node.children[1].clone()).unwrap()))),
                _ => return None,
            }
        _ => return None,
    }
}