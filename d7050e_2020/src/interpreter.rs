use crate::ast::*;

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn interpreter(expr:Box<Expr>) {
    let mut var_env:VecDeque<HashMap<String,(Term,bool)>> = VecDeque::new();
    let mut funcs:HashMap<String, Box<Expr>> = HashMap::new();
    let mut scope = HashMap::new();
    let key = "a".to_string();
    //let value = (Term::Ref(Box::new(Term::Num(5))),true);
    let value = (Term::Num(5),true);
    scope.insert(key,value);
    var_env.push_front(scope);
    let mut scope_access:usize = 0;
    println!("{:?}",stmnt_eval(expr,&mut var_env,&funcs,scope_access));
    println!("{:?}",var_env);
}

fn check_var(var:String, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
mut scope_access:usize) -> Option<(Term,bool)> {
    for map in var_env.iter().enumerate() {
        if map.1.contains_key(&var) && map.0 <= scope_access  {
            let ret = map.1.get(&var).unwrap();
            return Some(ret.clone());
        }
    }
    None
}

fn get_var_val(var:Term, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
mut scope_access:usize) -> Option<(Term,bool)> {
    match var {
        Term::Var(v) => {
            let v = check_var(v, var_env, scope_access);
            match v {
                Some(t) => {
                    match t.0 {
                        Term::Var(_) | Term::Ref(_) | Term::RefMut(_) => {
                            get_var_val(t.0, var_env, scope_access)
                        },
                        _ => Some(t)
                    }
                }
                None => None
            }
        }
        _ => Some((var,true))
    }
}

pub fn expr_eval(expr:Box<Expr>,
    mut var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
    funcs: &HashMap<String,Box<Expr>>,
    mut scope_access:usize) -> Result<(Term,bool),String> {
    match *expr {
        Expr::Boolean(b) => Ok((Term::Bool(b),true)),
        Expr::Number(i) => Ok((Term::Num(i),true)),
        Expr::Variable(v) => {
            let var_type = check_var(v.clone(), var_env, scope_access);
            match var_type {
                Some(var) => {
                    Ok((Term::Var(v),var.1))
                },
                None => Err("Variable not in scope".to_string())
            }
        },
        Expr::Infix(l,op,r) => {
            let l_eval = expr_eval(l,var_env,funcs,scope_access);
            let r_eval = expr_eval(r,var_env,funcs,scope_access);
            if l_eval.is_err() || r_eval.is_err() {
                return Err("Infix error".to_string());
            }
            let l_eval = get_var_val(l_eval.unwrap().0, var_env, scope_access);
            let r_eval = get_var_val(r_eval.unwrap().0, var_env, scope_access);

            match l_eval {
                Some((l_eval,_l_b)) => {
                    match r_eval {
                        Some((r_eval,_r_b)) => {
                            match op {
                                Opcode::Add => {
                                    Ok((Term::Num(l_eval.get_num().unwrap() + r_eval.get_num().unwrap()),true))
                                },
                                Opcode::Sub => {
                                    Ok((Term::Num(l_eval.get_num().unwrap() - r_eval.get_num().unwrap()),true))
                                },
                                Opcode::Mul => {
                                    Ok((Term::Num(l_eval.get_num().unwrap() * r_eval.get_num().unwrap()),true))
                                },
                                Opcode::Div => {
                                    Ok((Term::Num(l_eval.get_num().unwrap() / r_eval.get_num().unwrap()),true))
                                },
                                Opcode::Less => {
                                    Ok((Term::Bool(l_eval.get_num().unwrap() < r_eval.get_num().unwrap()),true))
                                },
                                Opcode::Greater => {
                                    Ok((Term::Bool(l_eval.get_num().unwrap() > r_eval.get_num().unwrap()),true))
                                },
                                Opcode::And => {
                                    Ok((Term::Bool(l_eval.get_bool().unwrap() && r_eval.get_bool().unwrap()),true))
                                },
                                Opcode::Or => {
                                    Ok((Term::Bool(l_eval.get_bool().unwrap() || r_eval.get_bool().unwrap()),true))
                                },
                                Opcode::Equals => {
                                    match l_eval {
                                        Term::Num(l) => {
                                            Ok((Term::Bool(l == r_eval.get_num().unwrap()),true))
                                        },
                                        Term::Bool(l) => {
                                            Ok((Term::Bool(l == r_eval.get_bool().unwrap()),true))
                                        },
                                        _ => Err("Type not applicable for Equals operation".to_string())
                                    }
                                },
                                _ => Err("Opcode isnt infix".to_string())
                            }
                        },
                        None => Err("Variable not in scope".to_string())
                    }
                },
                None => Err("Variable not in scope".to_string())
            }
        },
        Expr::Prefix(op, r) => {
            let r_eval = expr_eval(r,var_env,funcs,scope_access);
            if r_eval.is_err() {
                return r_eval
            }
            let r_eval = get_var_val(r_eval.unwrap().0, var_env, scope_access);
            match r_eval {
                Some((r_eval,_r_b)) => {
                    match op {
                        Opcode::Negate => Ok((Term::Num(-r_eval.get_num().unwrap()),true)),
                        Opcode::Not => Ok((Term::Bool(!r_eval.get_bool().unwrap()),true)),
                        _ => Err("Opcode isnt Prefix type".to_string())
                    }
                },
                None => Err("Variable not in scope".to_string())
            }
        },
        Expr::Unary(op,r) => {
            let rt_eval = expr_eval(r,var_env,funcs,scope_access);
            if rt_eval.is_err() {
                return rt_eval
            }
            let r_eval = get_var_val(rt_eval.clone().unwrap().0, var_env, scope_access);
            match r_eval {
                Some((r_eval,_r_b)) => {
                    match op {
                        Opcode::Ref => Ok((Term::Ref(Box::new(rt_eval.unwrap().0)),true)),
                        Opcode::RefMut => Ok((Term::RefMut(Box::new(rt_eval.unwrap().0)),true)),
                        Opcode::Deref => {
                            if r_eval.is_ref() {
                                Ok((r_eval.pop().unwrap(),false))
                            }
                            else if r_eval.is_refmut() {
                                Ok((r_eval.pop().unwrap(),true))
                            }
                            else {
                                Err("Cannot deref non Ref(Term)".to_string())
                            }
                        },
                        _=> unimplemented!("Implement FuncCall")
                    }
                },
                None => Err("Variable not in scope".to_string())
            }
        },
        _ => Err("Not an expr".to_string())
    }
}

pub fn stmnt_eval(expr:Box<Expr>,
    mut var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
    funcs: &HashMap<String,Box<Expr>>,
    mut scope_access:usize) -> Result<(Term,bool),String> {
    
    match *expr {
        Expr::Let(m,name,_type,expr) => {
            let r_eval = expr_eval(expr, var_env, funcs, scope_access);
            if r_eval.is_err() {
                return r_eval;
            }
            let r_eval = r_eval.unwrap();
            let mut scope = var_env.pop_front().unwrap();
            scope.insert(name, (r_eval.clone().0,m));
            var_env.push_front(scope);
            Ok(r_eval)
        },
        Expr::Assign(l,r) => {
            let r_eval = expr_eval(r, var_env, funcs, scope_access);
            let l_eval = expr_eval(l, var_env, funcs, scope_access);
            unimplemented!()
        },
        _ => expr_eval(expr, var_env, funcs, scope_access)
    }
}

/* fn remove_old_owner(var:Term, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
mut scope_access:usize) {
    for map in var_env.iter().enumerate() {
        if map.1.contains_key(&var) && map.0 <= scope_access  {
            map.1.remove(&var);
        }
    }
} */

fn block_eval() -> () {
    unimplemented!();
}