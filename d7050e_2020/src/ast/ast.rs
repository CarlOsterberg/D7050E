use std::fmt;

// ast

// println!("{:?}", ..)

#[derive(Debug,PartialEq,Clone)]
pub enum Term {
    Num(i32),
    Var(String),
    Bool(bool),
}

#[derive(Debug,PartialEq,Clone)]
pub enum Type {
    I32,
    Bool,
    Unit,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::I32 => "I32".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Unit => "Unit".to_string(),
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Boolean(bool),
    Infix(Box<Expr>, Opcode, Box<Expr>),
    Prefix(Opcode, Box<Expr>),
    Type(String),
    FuncCall(String, Vec<Box<Expr>>),
    Assign(String, Box<Expr>),
    Let(String,String,Type,Box<Expr>),
    While(Box<Expr>, Vec<Box<Expr>>),
    If(Box<Expr>,Vec<Box<Expr>>,Option<Vec<Box<Expr>>>),
    Func(String,Vec<(String,Type)>,Type,Vec<Box<Expr>>),
    Program(Vec<Box<Expr>>),
}

impl Expr {
    pub fn get(&self) -> (String,Vec<Type>) {
        match self {
            Expr::Func(name,params,ret_type,_scope) => {
                let mut ret_vec:Vec<Type> = Vec::new();
                for param in params {
                    ret_vec.push(param.1.clone());
                }
                ret_vec.push(ret_type.clone());
                (name.clone(), ret_vec)
            },
            _=> unimplemented!("get only implemented for Expr::Func()"),
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
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

impl Opcode {
    pub fn get_type(&self) -> Type {
        match self {
            Opcode::Mul 
            | Opcode::Div
            | Opcode::Add
            | Opcode::Sub
            | Opcode::Negate 
            => Type::I32,
            Opcode::And
            | Opcode::Or
            | Opcode::Equals
            | Opcode::Not
            | Opcode::Less
            | Opcode::Greater
            => Type::Bool
        }
    }
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