use crate::ast::*;

use std::collections::HashMap;
use std::collections::VecDeque;

fn check_env(name:String,map_vec:&mut VecDeque<HashMap<String,Type>>) -> Option<Type> {
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
    None
}

pub fn type_checker(prgrm:Vec<Box<Expr>>) {
    let mut scopes = VecDeque::new();
    let mut func_info:HashMap<String, Vec<String>> = HashMap::new();
    let map:HashMap<String,Type> = HashMap::new();
    scopes.push_front(map);
    for stmnt in &prgrm {
        let info = stmnt.get();
        func_info.insert(info.0, info.1);
    }
    println!("{:?}",&func_info);
    for stmnt in prgrm {
        println!("{:?}", expr_type(stmnt, &mut scopes, &func_info));
    }
}

//get Type of expr, stmnts are also expr
pub fn expr_type(e:Box<Expr>,mut var_env:&mut VecDeque<HashMap<String,Type>>
    ,func_info:&HashMap<String,Vec<String>>) -> Result<Type,String> {
    match *e {
        Expr::Number(_) => Ok(Type::I32),
        Expr::Boolean(_) => Ok(Type::Bool),
        Expr::Variable(name) => {
            let res = check_env(name, &mut var_env);
            match res {
                Some(res) => Ok(res),
                None => Err("Variable not in enviroment".to_string()),
            }
        },
        Expr::Infix(l, op, r) => {
            let lt = expr_type(l,&mut var_env,func_info)?;
            let rt = expr_type(r,&mut var_env,func_info)?;
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
            let rt = expr_type(r,&mut var_env,func_info)?;
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
            let rt = expr_type(eval, &mut var_env,func_info)?;
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
            let eval_res = expr_type(eval, &mut var_env, func_info)?;
            let res = check_env(name, &mut var_env);
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
        },
        Expr::While(expr_eval,block_eval) => {
            let lt = expr_type(expr_eval,&mut var_env,func_info)?;
            let rt = block_type(block_eval,&mut var_env,func_info);
            if lt==Type::Bool {
                rt
            }
            else {
                Err("While fuk".to_string())
            }
        },
        Expr::If(if_eval,if_block,
            else_content) => {
            let if_bool = expr_type(if_eval,&mut var_env,func_info)?;
            let l = block_type(if_block,&mut var_env,func_info)?;
            match else_content {
                Some(else_block) => {
                    let r = block_type(else_block,&mut var_env,func_info)?;
                    if r == l {
                        if if_bool == Type::Bool {
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
                    if if_bool == Type::Bool {
                        Ok(l)
                    }
                    else {
                        Err("in: (if <expr> {...}), <expr> didnt evaluate into a boolean.".to_string())
                    }
                },
            }
        },
        Expr::Func(_name, args, ret, block_eval) => {
            for  var in args {
                let mut map = var_env.pop_front().unwrap();
                map.insert(var.0, var.1);
                var_env.push_front(map);
            }
            let rt = block_type(block_eval,&mut var_env,func_info)?;
            if rt == ret {
                Ok(ret)
            }
            else {
                Err("Fn return type doesnt match the scope return type".to_string())
            }
        },
        Expr::FuncCall(name, params) => {
            let mut counter = 0;
            let func_args = func_info.get(&name);
            match func_args {
                Some(args) => {
                    if (args.len() - 1) == params.len() {
                        for param in params {
                            let param_type = expr_type(param,&mut var_env,&func_info)?;
                            if param_type.to_string() != args[counter] {
                                return Err("Paramater and argument type missmatch".to_string());
                            }
                            counter = counter + 1;
                        }
                        if args[counter] == String::from("I32") {
                            Ok(Type::I32)
                        }
                        else if args[counter] == String::from("Bool") {
                            Ok(Type::Bool)
                        }
                        else if args[counter] == String::from("Unit") {
                            Ok(Type::Unit)
                        }
                        else {
                            Err("Not a valid type".to_string())
                        }
                    }
                    else {
                        Err("funccall paramns and func arg doesnt match".to_string())
                    }
                }
                None => Err("Function doesnt exist.".to_string())
            }
        },
        _=> unimplemented!(),
    }
}
//get Type of a scope
pub fn block_type(mut block:Vec<Box<Expr>>,mut var_env:&mut VecDeque<HashMap<String,Type>>
,func_info:&HashMap<String,Vec<String>>) -> Result<Type,String> {
    let last = block.pop();
    let scope:HashMap<String, Type> = HashMap::new();
    var_env.push_front(scope);
    for stmnt in block {
        expr_type(stmnt,&mut var_env,func_info)?;
    }
    match last {
        Some(expr) => {
            match *expr {
                Expr::Number(_) => {
                    var_env.pop_front();
                    Ok(Type::I32)},
                Expr::Boolean(_) => {
                    var_env.pop_front();
                    Ok(Type::Bool)},
                Expr::Variable(name) => {
                    let res = check_env(name, &mut var_env);
                    match res {
                        Some(res) => {
                            var_env.pop_front();
                            Ok(res)},
                        None => Err("Variable not in enviroment".to_string()),
                    }
                },
                Expr::Infix(l, op, r) => {
                    let lt = expr_type(l,&mut var_env,func_info)?;
                    let rt = expr_type(r,&mut var_env,func_info)?;
                    match op {
                        //Operations with I32
                        Opcode::Add | Opcode::Mul | Opcode::Div 
                        | Opcode::Sub  => {
                            // check if op and args are compliant
                            let opt = op.get_type();
                            if lt == opt && rt == opt {
                                var_env.pop_front();
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
                                var_env.pop_front();
                                Ok(opt)
                            } else {
                                Err("Bool infix".to_string())
                            }
                        },
                        Opcode::Less | Opcode::Greater => {
                            let opt = op.get_type();
                            if lt == Type::I32 && rt == Type::I32 {
                                var_env.pop_front();
                                Ok(opt)
                            } else {
                                Err("Less, greater".to_string())
                            }
                        },
                        _ => Err("Operand not infix".to_string())
                    }
                },
                Expr::Prefix(op,r) => {
                    let rt = expr_type(r,&mut var_env,func_info)?;
                    match op {
                        Opcode::Negate => {
                            let opt = op.get_type();
                            if rt == opt {
                                var_env.pop_front();
                                Ok(opt)
                            } else {
                                Err("I32 prefix".to_string())
                            }
                        },
                        Opcode::Not => {
                            let opt = op.get_type();
                            if rt == opt {
                                var_env.pop_front();
                                Ok(opt)
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
                                    let param_type = expr_type(param,&mut var_env,&func_info)?;
                                    if param_type.to_string() != args[counter] {
                                        return Err("Paramater and argument type missmatch".to_string());
                                    }
                                    counter = counter + 1;
                                }
                                if args[counter] == String::from("I32") {
                                    Ok(Type::I32)
                                }
                                else if args[counter] == String::from("Bool") {
                                    Ok(Type::Bool)
                                }
                                else if args[counter] == String::from("Unit") {
                                    Ok(Type::Unit)
                                }
                                else {
                                    Err("Not a valid type".to_string())
                                }
                            }
                            else {
                                Err("funccall paramns and func arg doesnt match".to_string())
                            }
                        }
                        None => Err("Function doesnt exist.".to_string())
                    }
                },
                _=> {
                    var_env.pop_front();
                    Ok(Type::Unit)
                },
            }
        },
        None => {
            var_env.pop_front();
            Ok(Type::Unit)
        },
    }
}