use std::fmt;

// ast

// println!("{:?}", ..)
#[derive(Debug)]
pub enum Term {
    Num(i32),
    Var(String),
}
#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Infix(Box<Expr>, Opcode, Box<Expr>),
    Prefix(Opcode, Box<Expr>),
}
#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Negate,
    Less,
    Greater,
    And,
    Or,
    Equals,
    Not,
}

// println!("{}", ..)

