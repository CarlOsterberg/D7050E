use crate::ast::*;

use std::collections::HashMap;


pub fn expr_type(e:Box<Expr>,mut var_env:&mut HashMap<String,Type>
    ,param_env:&HashMap<String,Type>) -> Result<Type,String> {
    match *e {
        Expr::Number(_) => Ok(Type::I32),
        Expr::Boolean(_) => Ok(Type::Bool),
        Expr::Variable(name) => {
            if var_env.contains_key(&name) {
                let ret = var_env.get(&name).unwrap();
                match ret {
                    &Type::I32 => Ok(Type::I32),
                    &Type::Bool => Ok(Type::Bool),
                    _=> Err("Type is not valid for variable".to_string())
                }
            }
            else if param_env.contains_key(&name) {
                let ret = param_env.get(&name).unwrap();
                match ret {
                    &Type::I32 => Ok(Type::I32),
                    &Type::Bool => Ok(Type::Bool),
                    _=> Err("Type is not valid for variable".to_string())
                }
            }
            else {
                Err("Variable not in enviroment".to_string())
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
                var_env.insert(name, kind);
                Ok(rt)
            }
            else {
                Err("Let type and expr type doesnt match.".to_string())
            }
        },
        Expr::Assign(name,eval)=> {
            let eval_res = expr_type(eval, &mut var_env, &param_env)?;
            if var_env.contains_key(&name) {
                let ret = var_env.get(&name).unwrap();
                if eval_res == *ret {
                    Ok(eval_res)
                }
                else {
                    Err("Assign fukkd".to_string())
                }
            }
            else if param_env.contains_key(&name) {
                let ret = param_env.get(&name).unwrap();
                if eval_res == *ret {
                    Ok(eval_res)
                }
                else {
                    Err("Assign fukkd".to_string())
                }
            }
            else {
                Err("Variable not in enviroment".to_string())
            }
        },
        Expr::While(expr_eval,block_eval) => {
            let lt = expr_type(expr_eval,&mut var_env,param_env)?;
            let rt = block_type(block_eval,&mut var_env,&param_env);
            if lt==Type::Bool {
                rt
            }
            else {
                Err("While fuk".to_string())
            }
        },
        _=> unimplemented!(),
    }
}

pub fn block_type(mut block:Vec<Box<Expr>>,mut var_env:&mut HashMap<String,Type>
,param_env:&HashMap<String,Type>) -> Result<Type,String> {
    let last = block.pop();
    match last {
        None => Ok(Type::Unit),
        _=> {
            for stmnt in block {
                expr_type(stmnt,&mut var_env,param_env)?;
            }
            match *last.unwrap() {
                Expr::Number(_) => Ok(Type::I32),
                Expr::Boolean(_) => Ok(Type::Bool),
                Expr::Variable(name) => {
                    if var_env.contains_key(&name) {
                        let ret = var_env.get(&name).unwrap();
                        match ret {
                            &Type::I32 => Ok(Type::I32),
                            &Type::Bool => Ok(Type::Bool),
                            _=> Err("Type is not valid for variable".to_string())
                        }
                    }
                    else if param_env.contains_key(&name) {
                        let ret = param_env.get(&name).unwrap();
                        match ret {
                            &Type::I32 => Ok(Type::I32),
                            &Type::Bool => Ok(Type::Bool),
                            _=> Err("Type is not valid for variable".to_string())
                        }
                    }
                    else {
                        Err("Variable not in enviroment".to_string())
                    }
                },
                Expr::Infix(l, op, r) => {
                    let lt = expr_type(l,&mut var_env,param_env)?;
                    let rt = expr_type(r,&mut var_env,param_env)?;
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
                    let rt = expr_type(r,&mut var_env,param_env)?;
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
                _=> Ok(Type::Unit)
            }
        }
    }
}