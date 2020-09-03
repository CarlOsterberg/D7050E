use std::fmt;

// ast

// println!("{:?}", ..)
#[derive(Debug)]
pub enum NumOrId {
    Num(usize),
    Id(String),
    Expr(String),
    Op(String),
}

// println!("{}", ..)
impl fmt::Display for NumOrId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumOrId::Num(i) => write!(f, "{}", i)?,
            NumOrId::Id(s) => write!(f, "{}", s)?,
            NumOrId::Expr(s) => write!(f, "{}", s)?,
            NumOrId::Op(s) => write!(f, "{}", s)?,
        };
        Ok(())
    }
}
