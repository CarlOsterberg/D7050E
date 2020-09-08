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
    Op(Box<Expr>, Opcode, Box<Expr>),
}
#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Less,
    Greater,
    And,
    Or,
    Equals,
    Not,
}

// println!("{}", ..)

