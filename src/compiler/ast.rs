use std::fmt;

use crate::compiler::tokenizer::*;
use crate::Token::*;
use ValueType::*;
use NonLiteralName::*;


enum NonLiteralName {
    Program,
    Function,
    Statement,
    Expression,
}

enum ValueType {
    NonLiteral(NonLiteralName),
    Literal(Token),
}

pub struct Node {
    value_type: ValueType,
    children: Vec<Node>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node { value_type: Literal(t), .. } => write!(f, "{}", t),
            Node { value_type: NonLiteral(..), children: ch } => {
                for (.., v) in ch.iter().enumerate() {
                    let _ = write!(f, "{}", v);
                }
                write!{f, ""}
            },
            //_ => todo!(),
        }
    }
}

pub fn parse_program(tokens: Vec<Token>) -> Option<Node> {
    // <program> -> <function>
    let function = parse_function(tokens);
    match function {
        Some(node) => return Some(Node {value_type: NonLiteral(Program), children: vec![node]}),
        _ => return None,
    }
}

fn parse_function(tokens: Vec<Token>) -> Option<Node> {
    // <function> -> "int" <id> "(" ")" "{" <statement> "}" 
    let (int, id, leftp, rightp, leftb, statement, rightb);
    match tokens.get(0) {
        Some(Keyword(KeywordName::Int)) => int = Node { value_type: Literal(Keyword(KeywordName::Int)), children: vec![] },
        _ => return None,
    }
    match tokens.get(1) {
        Some(Identifier(i)) => id = Node { value_type: Literal(Identifier(i.to_string())), children: vec![] },
        _ => return None,
    }
    match tokens.get(2) {
        Some(OpenParenthesis) => leftp = Node { value_type: Literal(OpenParenthesis), children: vec![] },
        _ => return None,
    }
    match tokens.get(3) {
        Some(CloseParenthesis) => rightp = Node { value_type: Literal(CloseParenthesis), children: vec![] },
        _ => return None,
    }
    match tokens.get(4) {
        Some(OpenBrace) => leftb = Node { value_type: Literal(OpenBrace), children: vec![] },
        _ => return None,
    }
    match parse_statement(tokens[5..tokens.len()-1].to_vec()) {
        Some( Node { value_type: NonLiteral(Statement), children: ch } ) => statement = Node { value_type: NonLiteral(Statement), children: ch },
        _ => return None,
    }
    match tokens.get(tokens.len() - 1) {
        Some(CloseBrace) => rightb = Node { value_type: Literal(CloseBrace), children: vec![] },
        _ => return None,
    }
    return Some(Node { value_type: NonLiteral(Function), children: vec![int, id, leftp, rightp, leftb, statement, rightb] });

}

fn parse_statement(tokens: Vec<Token>) -> Option<Node> {
    // <statement> -> "return" <exp> ";"
    let (ret, exp, semic);
    match tokens.get(0) {
        Some(Keyword(KeywordName::Return)) => ret = Node { value_type: Literal(Keyword(KeywordName::Return)), children: vec![] },
        _ => return None,
    }
    match parse_exp(tokens[1..tokens.len() - 1].to_vec()) {
        Some( Node { value_type: NonLiteral(Expression), children: ch } ) => exp = Node { value_type: NonLiteral(Expression), children: ch },
        _ => return None,
    }
    match tokens.get(tokens.len() - 1) {
        Some(Semicolon) => semic = Node { value_type: Literal(Semicolon), children: vec![] },
        _ => return None,
    }
    return Some(Node { value_type: NonLiteral(Statement), children: vec![ret, exp, semic] });
}

fn parse_exp(tokens: Vec<Token>) -> Option<Node> {
    // <exp> -> <unary_op> <exp> | <int>
    let int;
    match tokens.get(0) {
        Some(IntegerLiteral(i)) => {
            int = Node { value_type: Literal(IntegerLiteral(*i)), children: vec![] };
            return Some(Node { value_type: NonLiteral(Expression), children: vec![int] })
        }
        Some(UnaryOperation(op)) => {
            let un = Node{ value_type: Literal(UnaryOperation(op.clone())), children: vec![] };
            let sub_exp = parse_exp(tokens[1..].to_vec());
            match sub_exp {
                Some(Node{ value_type: NonLiteral(Expression), .. }) => {
                    return Some(Node{ value_type: NonLiteral(Expression), children: vec![un, sub_exp.unwrap()] });
                }
                _ => return None,
            }
        }
        _ => return None,
    }
}
