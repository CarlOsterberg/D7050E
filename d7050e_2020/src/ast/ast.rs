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
    Ref(Box<Type>),
    Mut(Box<Type>),
    Var(String),
}


impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::I32 => "I32".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Unit => "Unit".to_string(),
            Type::Ref(t) => {
                let a = t.to_string();
                let mut ret = "Ref(".to_string();
                ret.push_str(&a);
                ret.push_str(")");
                ret
            },
            Type::Mut(t) => {
                let a = t.to_string();
                let mut ret = "Mut(".to_string();
                ret.push_str(&a);
                ret.push_str(")");
                ret
            },
            Type::Var(t) => {
                let mut ret = "Var(".to_string();
                ret.push_str(&t);
                ret.push_str(")");
                ret
            },
        }
    }

    pub fn is_mut(&self) -> bool {
        match self {
            Type::Mut(_) => true,
            _ => false
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Type::Ref(_) => true,
            _ => false
        }
    }

    pub fn is_ref_var(&self)-> bool{
        match self {
            Type::Ref(ret) | Type::Mut(ret) => {
                ret.is_ref_var()
            }
            Type::Var(_) => true,
            _ => false
        }
    }
    

    pub fn is_var(&self) -> bool {
        match self {
            Type::Var(_) => true,
            _ => false
        }
    }

    pub fn get_var(&self) -> Option<String> {
        match self {
            Type::Var(n) => Some(n.to_string()),
            _ => None
        }
    }

    pub fn pop_ref(self) -> Type {
        match self {
            Type::Mut(ret) => {
                Type::Mut(Box::new(ret.pop_ref()))
            },
            Type::Ref(ret) => {
                *ret
            }
            _ => self,
        }
    }

    pub fn pop_mut(self) -> Type {
        match self {
            Type::Mut(ret) => {
                *ret
            },
            _ => self,
        }
    }
    pub fn pop_inner_mut(self) -> Type {
        let t = self.inner();
        t.0
    }

    fn inner(self) -> (Type,bool) {
        match self {
            Type::Mut(next) => {
                let t = next.inner();
                if t.1 {
                    (t.0,false)
                }
                else {
                    (Type::Mut(Box::new(t.0)),false)
                }
            },
            Type::Ref(next) => {
                let t = next.inner();
                (Type::Ref(Box::new(t.0)),t.1)

            },
            _=> (self,true)
        }
    }

    pub fn traverse(&self) -> &Type {
        match self {
            Type::Mut(next) => next.traverse(),
            Type::Ref(next) => next.traverse(),
            _=> self
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
    Unary(Opcode, Box<Expr>),
    Type(String),
    FuncCall(String, Vec<Box<Expr>>),
    Assign(Box<Expr>, Box<Expr>),
    Let(String,Type,Box<Expr>),
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
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Equals,
    NotEquals,
    Not,
    Ref,
    Mut,
    MutRef,
    Deref,
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
            | Opcode::NotEquals
            | Opcode::Not
            | Opcode::Less
            | Opcode::LessEqual
            | Opcode::Greater
            | Opcode::GreaterEqual
            => Type::Bool,
            _=> unimplemented!(),
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
            Opcode::LessEqual => write!(f, "<="),
            Opcode::Greater => write!(f, ">"),
            Opcode::GreaterEqual => write!(f, ">="),
            Opcode::Or => write!(f, "||"),
            Opcode::And => write!(f, "&&"),
            Opcode::Equals => write!(f, "=="),
            Opcode::NotEquals => write!(f, "!="),
            _=> unimplemented!(),
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