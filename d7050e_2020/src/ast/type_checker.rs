use crate::ast::*;

use std::collections::HashMap;
use std::collections::VecDeque;

fn check_env(name:String, map_vec:&mut VecDeque<HashMap<String,Type>>
    ,fn_map:&HashMap<String,Type>) -> Option<Type> {
    for map in map_vec {
        if map.contains_key(&name) {
            let ret = map.get(&name).unwrap();
            if *ret == Type::I32 {
                return Some(Type::I32);
            }
            else if *ret == Type::Bool {
                return Some(Type::Bool);
            }
            else {
                return None;
            }

        }
    }
    if fn_map.contains_key(&name) {
        let ret = fn_map.get(&name).unwrap();
        match ret {
            &Type::I32 => Some(Type::I32),
            &Type::Bool => Some(Type::Bool),
            _=> None
        }
    }
    else {
        None
    }
}

pub fn expr_type(e:Box<Expr>,mut var_env:&mut VecDeque<HashMap<String,Type>>
    ,param_env:&HashMap<String,Type>) -> Result<Type,String> {
    match *e {
        Expr::Number(_) => Ok(Type::I32),
        Expr::Boolean(_) => Ok(Type::Bool),
        Expr::Variable(name) => {
            let res = check_env(name, &mut var_env, &param_env);
            match res {
                Some(res) => Ok(res),
                None => Err("Variable not in enviroment".to_string()),
            }
        },
        Expr::Infix(l, op, r) => {
            let lt = expr_type(l,&mut var_env,&param_env)?;
            let rt = expr_type(r,&mut var_env,&param_env)?;
            match op {
                //Operations with I32
                Opcode::Add | Opcode::Mul | Opcode::Div 
                | Opcode::Sub  => {
                    // check if op and args are compliant
                    let opt = op.get_type();
                    if lt == opt && rt == opt {
                        Ok(opt)
                    } else {
                        Err("I32 infix".to_string())
                    }
                },
                //Or, and, equals can have different type from op
                Opcode::Or | Opcode::And | Opcode::Equals => {
                    // both sides need to be of same type
                    let opt = op.get_type();
                    if lt == rt {
                        Ok(opt)
                    } else {
                        Err("Bool infix".to_string())
                    }
                },
                Opcode::Less | Opcode::Greater => {
                    let opt = op.get_type();
                    if lt == Type::I32 && rt == Type::I32 {
                        Ok(opt)
                    } else {
                        Err("Less, greater".to_string())
                    }
                },
                _ => Err("Operand not infix".to_string())
            }
        },
        Expr::Prefix(op,r) => {
            let rt = expr_type(r,&mut var_env,&param_env)?;
            match op {
                Opcode::Negate => {
                    let opt = op.get_type();
                    if rt == opt {
                        Ok(opt)
                    } else {
                        Err("I32 prefix".to_string())
                    }
                },
                Opcode::Not => {
                    let opt = op.get_type();
                    if rt == opt {
                        Ok(opt)
                    } else {
                        Err("Bool prefix".to_string())
                    }
                },
                _=> Err("Operand not prefix".to_string())

            }
        },
        Expr::Let(_read, name, kind, eval) => {
            let rt = expr_type(eval, &mut var_env,&param_env)?;
            if rt == kind {
                let mut map = var_env.pop_front().unwrap();
                map.insert(name, kind);
                var_env.push_front(map);
                Ok(rt)
            }
            else {
                Err("Let type and expr type doesnt match.".to_string())
            }
        },
        Expr::Assign(name,eval)=> {
            let eval_res = expr_type(eval, &mut var_env, &param_env)?;
            let res = check_env(name, &mut var_env, &param_env);
            match res {
                Some(thing) => {
                    if eval_res == thing {
                        Ok(eval_res)
                    }
                    else {
                        Err("Type missmatch".to_string())
                    }
                },
                None => Err("The assigned variable doesnt exist in the enviroment.".to_string()),
            }
        },/*
        Expr::While(expr_eval,block_eval) => {
            let lt = expr_type(expr_eval,&mut var_env,param_env)?;
            let rt = block_type(block_eval,&mut var_env,&param_env);
            if lt==Type::Bool {
                rt
            }
            else {
                Err("While fuk".to_string())
            }
        },*/
        _=> unimplemented!(),
    }
}
/*
pub fn block_type(mut block:Vec<Box<Expr>>,mut var_env:&mut VecDeque<HashMap<String,Type>>
,param_env:&HashMap<String,Type>) -> Result<Type,String> {
    let last = block.pop();
    
}*/