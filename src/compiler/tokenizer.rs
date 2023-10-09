use std::fmt;

use crate::compiler::tokenizer::Token::*;
use std::str::FromStr;


#[derive(Clone, Debug)]
pub enum KeywordName {
    Return,
    Int,
}

#[derive(Clone, Debug)]
pub enum UnaryOperationName {
    Negation,
    BitwiseComp,
    LogNeg,
}

#[derive(Clone, Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    Keyword(KeywordName),
    Identifier(String),
    IntegerLiteral(i32),
    UnaryOperation(UnaryOperationName),
}

impl fmt::Display for KeywordName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeywordName::Int => write!(f, "int "),
            KeywordName::Return => write!(f, "return "),
        }
    }
}

impl fmt::Display for UnaryOperationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperationName::Negation => write!(f, "-"),
            UnaryOperationName::BitwiseComp => write!(f, "~"),
            UnaryOperationName::LogNeg => write!(f, "!"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenBrace => write!(f, "\n{{\n\t"),
            CloseBrace => write!(f, "\n}}"),
            OpenParenthesis => write!(f, "("),
            CloseParenthesis => write!(f, ")"),
            Semicolon => write!(f, ";"),
            Keyword(i) => write!(f, "{}", i),
            Identifier(s) => write!(f, "{}", s),
            IntegerLiteral(i) => write!(f, "{}", i),
            UnaryOperation(i) => write!(f, "{}", i),
            //_ => todo!(),
        }
    }
}

const ENDING_TOKENS: &'static [char] = &['{', '}', '(', ')', ';', '-', '~', '!'];

fn is_ending(c: char) -> bool {
    if c.is_whitespace() {
        return true;
    }
    if ENDING_TOKENS.contains(&c) {
        return true;
    }
    return false;
}

type State = String;

pub fn tokenize(code : String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut state: State = "".to_string();
    for symbol in code.chars() {
        if !is_ending(symbol) {
            state.push(symbol);
        }
        else {
            match state.as_str() {
                "" => (),
                "int" => tokens.push(Keyword(KeywordName::Int)),
                "return" => tokens.push(Keyword(KeywordName::Return)),
                state if state.chars().nth(0).unwrap().is_alphabetic() => tokens.push(Identifier(state.to_string())),
                state if state.chars().nth(0).unwrap().is_numeric() => tokens.push(IntegerLiteral(FromStr::from_str(state).unwrap())),
                _ => todo!(),
            }
            state = "".to_string();
            
            match symbol {
                '{' => tokens.push(OpenBrace),
                '}' => tokens.push(CloseBrace),
                '(' => tokens.push(OpenParenthesis),
                ')' => tokens.push(CloseParenthesis),
                ';' => tokens.push(Semicolon),
                '-' => tokens.push(UnaryOperation(UnaryOperationName::Negation)),
                '~' => tokens.push(UnaryOperation(UnaryOperationName::BitwiseComp)),
                '!' => tokens.push(UnaryOperation(UnaryOperationName::LogNeg)),
                _ => (),
            }
        }
    }
    return tokens;
}

