use crate::ast::*;

use std::collections::HashMap;
use std::collections::VecDeque;

fn check_env(name:String,map_vec:&mut VecDeque<HashMap<String,(Type,bool)>>) -> Option<(Type,bool)> {
    for map in map_vec {
        if map.contains_key(&name) {
            let ret = map.get(&name).unwrap();
            return Some(ret.clone());
        }
    }
    None
}

pub fn type_checker(prgrm:Vec<Box<Expr>>) -> Vec<Result<Type,String>> {
    let mut scopes:VecDeque<HashMap<String,(Type,bool)>> = VecDeque::new();
    let mut func_info:HashMap<String, Vec<(Type,bool)>> = HashMap::new();
    for stmnt in &prgrm {
        let info = stmnt.get();
        func_info.insert(info.0, info.1);
    }
    let mut ret_vec = Vec::new();
    for stmnt in prgrm {
        ret_vec.push(stmnt_type(stmnt, &mut scopes, &func_info));
    }
    ret_vec
}

pub fn stmnt_type(e:Box<Expr>,mut var_env:&mut VecDeque<HashMap<String,(Type,bool)>>
    ,func_info:&HashMap<String,Vec<(Type,bool)>>) -> Result<Type,String> {
        match *e {
            Expr::Let(m, name, kind, eval) => {
                let rt:(Type,bool);
                if eval.is_if() {
                    let res = stmnt_type(eval, var_env, func_info);
                    if res.is_ok() {
                        rt = (res.unwrap(),true);
                    }
                    else {
                        return res;
                    }
                }
                else {
                    rt = expr_type(eval, var_env,func_info)?;
                }
                if rt.0 == kind && (rt.1 == m || !m) {
                    let mut map = var_env.pop_front().unwrap();
                    map.insert(name, (kind,m));
                    var_env.push_front(map);
                    Ok(rt.0)
                }
                else {
                    Err("Let type and expr type doesnt match.".to_string())
                }
            },
            Expr::Assign(l,r)=> {
                let rt = expr_type(r, var_env, func_info);
                let lt = expr_type(l, var_env, func_info);
                if lt.is_err() {
                    return Err("Assign failed".to_string());
                }
                let lt =lt.unwrap();
                match rt {
                    Ok(thing) => {
                        if (lt.0 == thing.0) && (lt.1) {
                            Ok(lt.0)
                        }
                        else {
                            Err("Type missmatch".to_string())
                        }
                    },
                    Err(_) => Err("The assigned variable doesnt exist in the enviroment.".to_string()),
                }
            },
            Expr::While(expr_eval,block_eval) => {
                let lt = expr_type(expr_eval,var_env,func_info)?;
                let rt = block_type(block_eval,var_env,func_info);
                if lt.0==Type::Bool {
                    rt
                }
                else {
                    Err("While fuk".to_string())
                }
            },
            Expr::If(if_eval,if_block,
                else_content) => {
                let if_bool = expr_type(if_eval,var_env,func_info)?;
                let l = block_type(if_block,var_env,func_info)?;
                match else_content {
                    Some(else_block) => {
                        let r = block_type(else_block,var_env,func_info)?;
                        if r == l {
                            if if_bool.0 == Type::Bool {
                                Ok(r)
                            }
                            else {
                                Err("in: (if <expr> {...} else {...}), <expr> didnt evaluate into a boolean.".to_string())
                            }
                        }
                        else {
                            Err("in: (if <expr> {...} else {...}), the if and else scopes didnt return the same Type".to_string())
                        }
                    },
                    None => {
                        if if_bool.0 == Type::Bool {
                            Ok(l)
                        }
                        else {
                            Err("in: (if <expr> {...}), <expr> didnt evaluate into a boolean.".to_string())
                        }
                    },
                }
            },
            Expr::Func(_name, args, ret, block_eval) => {
                let m: HashMap<String,(Type,bool)> = HashMap::new();
                var_env.push_front(m);
                for  var in args {
                    let mut map = var_env.pop_front().unwrap();
                    map.insert(var.0, var.1);
                    var_env.push_front(map);
                }
                let rt = block_type(block_eval,var_env,func_info)?;
                var_env.pop_front();
                if rt == ret {
                    Ok(ret)
                }
                else {
                    Err("Fn return type doesnt match the scope return type".to_string())
                }
            },
            _=> {
                let r = expr_type(e,&mut var_env,func_info);
                match r {
                    Ok(rr) => {
                        Ok(rr.0)
                    },
                    Err(rr) => {
                        Err(rr)
                    },
                }
            },
        }
}

//get Type of expr, stmnts are also expr
pub fn expr_type(e:Box<Expr>,var_env:&mut VecDeque<HashMap<String,(Type,bool)>>
    ,func_info:&HashMap<String,Vec<(Type,bool)>>) -> Result<(Type,bool),String> {
    match *e {
        Expr::Number(_) => Ok((Type::I32,true)),
        Expr::Boolean(_) => Ok((Type::Bool,true)),
        Expr::Variable(name) => {
            let res = check_env(name, var_env);
            match res {
                Some(res) => Ok((res.0,res.1)),
                None => Err("Variable not in enviroment".to_string()),
            }
        },
        Expr::Infix(l, op, r) => {
            let lt = expr_type(l,var_env,func_info)?;
            let rt = expr_type(r,var_env,func_info)?;
            match op {
                //Operations with I32
                Opcode::Add | Opcode::Mul | Opcode::Div 
                | Opcode::Sub  => {
                    // check if op and args are compliant
                    let opt = op.get_type();
                    if lt.0 == opt && rt.0 == opt {
                        Ok((opt,true))
                    } else {
                        Err("I32 infix".to_string())
                    }
                },
                //Or, and, equals can have different type from op
                Opcode::Or | Opcode::And | Opcode::Equals => {
                    // both sides need to be of same type
                    let opt = op.get_type();
                    if lt.0 == rt.0 {
                        Ok((opt,true))
                    } else {
                        Err("Bool infix".to_string())
                    }
                },
                Opcode::Less | Opcode::Greater => {
                    let opt = op.get_type();
                    if lt.0 == Type::I32 && rt.0 == Type::I32 {
                        Ok((opt,true))
                    } else {
                        Err("Less, greater".to_string())
                    }
                },
                _ => Err("Operand not infix".to_string())
            }
        },
        Expr::Prefix(op,r) => {
            let rt = expr_type(r,var_env,func_info)?;
            match op {
                Opcode::Negate => {
                    let opt = op.get_type();
                    if rt.0 == opt {
                        Ok((opt,true))
                    } else {
                        Err("I32 prefix".to_string())
                    }
                },
                Opcode::Not => {
                    let opt = op.get_type();
                    if rt.0 == opt {
                        Ok((opt,true))
                    } else {
                        Err("Bool prefix".to_string())
                    }
                },
                _=> Err("Operand not prefix".to_string())

            }
        },
        Expr::FuncCall(name, params) => {
            let mut counter = 0;
            let func_args = func_info.get(&name);
            match func_args {
                Some(args) => {
                    if (args.len() - 1) == params.len() {
                        for param in params {
                            let param_type = expr_type(param,var_env,func_info)?;
                            if param_type.0 != args[counter].0 || (param_type.1 == false && args[counter].1 == true) {
                                return Err("Parameter and argument type missmatch".to_string());
                            }
                            counter = counter + 1;
                        }
                        Ok(args[counter].clone())
                    }
                    else {
                        Err("funccall paramns and func arg doesnt match".to_string())
                    }
                }
                None => Err("Function doesnt exist.".to_string())
            }
        },
        Expr::Unary(op,r) => {
            let rt = expr_type(r,var_env,func_info)?;
            match op {
                Opcode::Ref => {
                    Ok((Type::Ref(Box::new(rt.0)),rt.1))
                },
                Opcode::RefMut => {
                    if rt.1 {
                        Ok((Type::RefMut(Box::new(rt.0)),true))
                    }
                    else {
                        Err("Cannot create mutable ref to immbutable".to_string())
                    }
                },
                Opcode::Deref => {
                    match rt.0 {
                        Type::Ref(c) => {
                            Ok((*c,false))
                        },
                        Type::RefMut(c) => {
                            Ok((*c,true))
                        },
                        _ => Err("Cant deref non ref".to_string())
                    }
                },
                _ => Err("Not unary op".to_string())
            }
        },
        _=> Err("Not a stmnt or expr".to_string()),
    }
}
//get Type of a scope
pub fn block_type(mut block:Vec<Box<Expr>>,var_env:&mut VecDeque<HashMap<String,(Type,bool)>>
,func_info:&HashMap<String,Vec<(Type,bool)>>) -> Result<Type,String> {
    let last = block.pop();
    let scope:HashMap<String, (Type,bool)> = HashMap::new();
    var_env.push_front(scope);
    for stmnt in block {
        let i = stmnt_type(stmnt,var_env,func_info);
        if i.is_err() {
            return i;
        }
    }
    match last {
        Some(expr) => {
            match *expr.clone() {
                Expr::If(_,_,_) => {
                    let res = stmnt_type(expr, var_env, func_info);
                    match res {
                        Ok(r) => {
                            var_env.pop_front();
                            Ok(r)
                        },
                        Err(context) => {
                            var_env.pop_front();
                            Err(context)
                        },
                    }
                },
                _ => {
                    let res = expr_type(expr.clone(), var_env, func_info);
                    match res {
                        Ok(r) => {
                            var_env.pop_front();
                            Ok(r.0)
                        },
                        Err(context) => {
                            if context == "Not a stmnt or expr".to_string() {
                                let stmnt_res = stmnt_type(expr, var_env, func_info);
                                var_env.pop_front();
                                match stmnt_res {
                                    Ok(_) => Ok(Type::Unit),
                                    Err(err) => Err(err),
                                }
                            }
                            else {
                                Err(context)
                            }
                        },
                    }
                }
            }
        },
        None => {
            var_env.pop_front();
            Ok(Type::Unit)
        },
    }
}