//use std::fmt;

// ast

// println!("{:?}", ..)

#[derive(Debug)]
pub enum Stmnt {
    Let(Readability,String,VarType,Box<Expr>),
    //If(StmntType,Box<Expr>,Box<Stmnt>),
    //While(StmntType,Box<Expr>,Box<Stmnt>),
}
#[derive(Debug)]
pub enum Readability {
    Read,
    Mut,
}
#[derive(Debug)]
pub enum VarType {
    I32,
    Bool,
}

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
/*
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(i) => write!(f, "{}", i)?,
            Expr::Variable(s) => write!(f, "{}", s)?,
            Expr::Infix() => write!(f, "{}", )?,
            Expr::Prefix() => write!(f, "{}", )?,
        };
        Ok(())
    }
}
*/