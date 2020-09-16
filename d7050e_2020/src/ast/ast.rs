use std::fmt;

// ast

// println!("{:?}", ..)


#[derive(Debug)]
pub enum Term {
    Num(i32),
    Var(String),
    Bool(bool),
}
#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Boolean(bool),
    Infix(Box<Expr>, Opcode, Box<Expr>),
    Prefix(Opcode, Box<Expr>),
    Type(String),
    FuncCall(String, Vec<Box<Expr>>),
    Assign(String, Box<Expr>),
    Let(String,String,String,Box<Expr>),
    While(Box<Expr>, Vec<Box<Expr>>),
    If(Box<Expr>,Vec<Box<Expr>>,Option<Vec<Box<Expr>>>),
    Func(String,Vec<(String,String)>,String,Vec<Box<Expr>>),
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

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Not => write!(f, "!"),
            Opcode::Negate => write!(f, "-"),
            Opcode::Less => write!(f, "<"),
            Opcode::Greater => write!(f, ">"),
            Opcode::Or => write!(f, "||"),
            Opcode::And => write!(f, "&&"),
            Opcode::Equals => write!(f, "=="),
        }?;
        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(i) => write!(f, "{}", i)?,
            Expr::Variable(s) => write!(f, "{}", s)?,
            Expr::Boolean(b) => write!(f, "{}", b)?,
            Expr::Infix(a,b,c) => write!(f, "({} {} {})", format!("{}", a), format!("{}", b), format!("{}", c))?,
            Expr::Prefix(a,b) => write!(f, "({} {})", format!("{}", a), format!("{}", b))?,
            //Expr::Let(a,b,c,d) => write!(f, "({} {} :{} ={};)", format!("{}", a), format!("{}", b), format!("{}", c), format!("{}", d))?,
            _ => panic!("error"),
        };
        Ok(())
    }
}