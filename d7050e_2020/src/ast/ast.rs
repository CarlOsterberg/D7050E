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
pub enum Root {
    Number(i32),
    Variable(String),
    Boolean(bool),
    Infix(Box<Root>, Opcode, Box<Root>),
    Prefix(Opcode, Box<Root>),
    Assign(String, Box<Root>,Option<Box<Root>>),
    Let(String,String,String,Box<Root>,Option<Box<Root>>),
    While(Box<Root>,Box<Root>,Option<Box<Root>>),
    If(Box<Root>,Box<Root>,Option<Box<Root>>,Option<Box<Root>>),
    Func(String, Vec<(String,String)>, String, Box<Root>, Option<Box<Root>>),
}

impl Root {
    pub fn next(&mut self, next_root: Root) {
        match *self {
            Root::While(.., ref mut next)
            |Root::Let(.., ref mut next) 
            |Root::Assign(.., ref mut next)
            |Root::If(.., ref mut next)=> {
                *next = Some(Box::new(next_root))
            }
            _ => panic!("Das root be fukkd"),
        };
    }
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

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Root::Number(i) => write!(f, "{}", i)?,
            Root::Variable(s) => write!(f, "{}", s)?,
            Root::Boolean(b) => write!(f, "{}", b)?,
            Root::Infix(a,b,c) => write!(f, "({} {} {})", format!("{}", a), format!("{}", b), format!("{}", c))?,
            Root::Prefix(a,b) => write!(f, "({} {})", format!("{}", a), format!("{}", b))?,
            //Root::Let(a,b,c,d) => write!(f, "({} {} :{} ={};)", format!("{}", a), format!("{}", b), format!("{}", c), format!("{}", d))?,
            _ => panic!("error"),
        };
        Ok(())
    }
}