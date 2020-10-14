use std::fmt;

// ast

// println!("{:?}", ..)

#[derive(Debug,PartialEq,Clone)]
pub enum Term {
    Num(i32),
    Var(String),
    Bool(bool),
    Ref(Box<Term>),
    RefMut(Box<Term>),
}

impl Term {
    pub fn get_num(self) -> Option<i32> {
        match self {
            Term::Num(i) => {
                Some(i)
            },
            _ => None
        }
    }
    pub fn get_bool(self) -> Option<bool> {
        match self {
            Term::Bool(b) => {
                Some(b)
            },
            _ => None
        }
    }
    pub fn is_ref(&self) -> bool {
        match self {
            Term::Ref(_) => true,
            _ => false
        }
    }
    pub fn is_refmut(&self) -> bool {
        match self {
            Term::RefMut(_) => true,
            _ => false
        }
    }
    pub fn pop(self) -> Result<Term,String> {
        match self {
            Term::RefMut(t) => Ok(*t),
            Term::Ref(t) => Ok(*t),
            _ => Err("Cannot deref non Ref(Term)".to_string())
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub enum Type {
    I32,
    Bool,
    Unit,
    Ref(Box<Type>),
    RefMut(Box<Type>)
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::I32 => "I32".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Unit => "Unit".to_string(),
            Type::Ref(t) | Type::RefMut(t)=> {
                let a = t.to_string();
                let mut ret = "Ref(".to_string();
                ret.push_str(&a);
                ret.push_str(")");
                ret
            },
            
        }
    }
    pub fn is_refmut(&self) -> bool {
        match self {
            Type::RefMut(_) => true,
            _ => false
        }
    }
    pub fn is_ref(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false
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
    Assign(Box<Expr>, Box<Expr>),
    Let(bool,String,Type,Box<Expr>),
    While(Box<Expr>, Vec<Box<Expr>>),
    If(Box<Expr>,Vec<Box<Expr>>,Option<Vec<Box<Expr>>>),
    Func(String,Vec<(String,(Type,bool))>,Type,Vec<Box<Expr>>),
    Program(Vec<Box<Expr>>),
    Unary(Opcode, Box<Expr>),
}

impl Expr {
    pub fn get(&self) -> (String,Vec<(Type,bool)>) {
        match self {
            Expr::Func(name,params,ret_type,_scope) => {
                let mut ret_vec:Vec<(Type,bool)> = Vec::new();
                for param in params {
                    ret_vec.push(param.1.clone());
                }
                ret_vec.push((ret_type.clone(),false));
                (name.clone(), ret_vec)
            },
            _=> unimplemented!("get only implemented for Expr::Func()"),
        }
    }
    pub fn var_get(self) -> Result<String,String> {
        match self {
            Expr::Variable(c) => Ok(c),
            _=> Err("Expr not var".to_string())
        }
    }
    pub fn is_if(&self) -> bool {
        match self {
            Expr::If(_,_,_) => true,
            _=> false
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
    Ref,
    Mut,
    Deref,
    RefMut,
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
            => Type::Bool,
            Opcode::Deref |
            Opcode::Mut |
            Opcode::Ref |
            Opcode::RefMut => Type::Unit
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
            Opcode::Deref => write!(f, "*"),
            Opcode::Ref => write!(f, "&"),
            Opcode::RefMut => write!(f, "&mut "),
            Opcode::Mut => write!(f, "mut "),
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