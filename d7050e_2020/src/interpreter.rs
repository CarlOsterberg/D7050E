use crate::ast::*;

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn interpreter(program:Vec<Box<Expr>>) {
    let mut var_env:VecDeque<HashMap<String,(Term,bool)>> = VecDeque::new();
    let mut funcs:HashMap<String, Box<Expr>> = HashMap::new();
    let scope = HashMap::new();
    var_env.push_front(scope);
    let mut scope_access:usize = 1;
    let mut main:Result<Box<Expr>,String> = Err("No main function exists".to_string());
    for func in program {
        let key = match *func.clone() {
            Expr::Func(name,_,_,_) => {
                Ok(name)
            },
            _ => {
                Err("Not a func".to_string())
            }
        };
        if key.is_err() {
            return;
        }
        let key = key.unwrap();
        if key == "main".to_string() {
            main = Ok(func.clone());
        }
        funcs.insert(key, func);
    }
    println!("{:?}",exe_func(main.unwrap(), &mut var_env, &funcs,&mut scope_access));
}

fn is_refed(var:String, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
scope_access:&mut usize) -> bool {

    for map in var_env.iter().enumerate() {
        if map.0 < *scope_access {
            for key in map.1.values() {
                if key.clone().0.is_ref() && key.clone().0.get_var().unwrap() == var {
                    return true;
                }
            }
        }
    }
    false
}

fn is_refmuted(var:String, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
scope_access:&mut usize) -> bool {

    for map in var_env.iter().enumerate() {
        if map.0 < *scope_access {
            for key in map.1.values() {
                if key.clone().0.is_refmut() && key.clone().0.get_var().unwrap() == var {
                    return true;
                }
            }
        }
    }
    false
}

fn check_var(var:String, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
scope_access:&mut usize) -> Option<(Term,bool)> {
    let mut try_name = var.clone();
    let mut extra_scopes = 0;
    while try_name.pop().unwrap() == '!' {
        extra_scopes = extra_scopes + 1;
    }
    if extra_scopes>0 {
        try_name = var.clone();
        let mut i = 0;
        while i<extra_scopes {
            try_name.pop();
            i = i + 1;
        }
        for map in var_env.iter().enumerate() {
            if map.1.contains_key(&try_name) && map.0 == *scope_access + extra_scopes - 1  {
                let ret = map.1.get(&try_name).unwrap();
                return Some(ret.clone());
            }
        }
    }
    else {
        for map in var_env.iter().enumerate() {
            if map.1.contains_key(&var) && map.0 < *scope_access  {
                let ret = map.1.get(&var).unwrap();
                return Some(ret.clone());
            }
        }
    }
    None
}

fn insert_assign(key:String,value:(Term,bool),var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
scope_access:&mut usize) -> Result<String,String> {

    
    let mut iter_count = 0;
    let mut try_name = key.clone();
    let mut extra_scopes = 0;
    while try_name.pop().unwrap() == '!' {
        extra_scopes = extra_scopes + 1;
    }
    if extra_scopes>0 {
        try_name = key.clone();
        let mut i = 0;
        while i<extra_scopes {
            try_name.pop();
            i = i + 1;
        }
        for map in var_env {
            if map.contains_key(&try_name) && iter_count == *scope_access + extra_scopes - 1 {
                map.insert(try_name, value);
                return Ok("Variable found and changed".to_string());
            }
            iter_count = iter_count + 1;
        }
    }
    else {
        for map in var_env {
            if map.contains_key(&key) && iter_count < *scope_access {
                map.insert(key, value);
                return Ok("Variable found and changed".to_string());
            }
            iter_count = iter_count + 1;
        }
    }
    Err("Variable not in scope".to_string())
}

fn get_var_val(var:Term, var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
scope_access:&mut usize) -> Option<(Term,bool)> {
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

fn get_func_arg_names(func:String,funcs: &HashMap<String,Box<Expr>>) -> Option<Vec<String>> {
    
    let mut func_params = Vec::new();
    for f in funcs {
        if f.0.to_string() == func {
            match *f.1.clone() {
                Expr::Func(_,params,_,_) => {
                    for param in params {
                        func_params.push(param.0);
                    }
                    return Some(func_params);
                },
                _ => {return None;}
            };
        }
    }
    None
}

fn get_func_expr(func:String,funcs: &HashMap<String,Box<Expr>>) -> Option<Box<Expr>> {
    for f in funcs {
        if *f.0.clone() == func {
            return Some(f.1.clone());
        }
    }
    None
}

fn exe_func(func:Box<Expr>,
    var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
    funcs: &HashMap<String,Box<Expr>>,
    scope_access:&mut usize) -> Result<(Term,bool),String> {

    match *func {
        Expr::Func(_name,_args,_ret_type,func_block) => {
            let func_eval = block_eval(func_block, var_env, funcs, scope_access);
            func_eval
        },
        _ => Err("Not a function".to_string())
    }
}

pub fn expr_eval(expr:Box<Expr>,
    var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
    funcs: &HashMap<String,Box<Expr>>,
    scope_access:&mut usize) -> Result<(Term,bool),String> {
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
                Some((r_eval,r_b)) => {
                    match op {
                        Opcode::Ref => {
                            if rt_eval.clone().unwrap().0.is_var_recur() {
                                if is_refmuted(rt_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) {
                                    return Err("Variable already has a mutable reference".to_string());
                                }
                            }
                            Ok((Term::Ref(Box::new(rt_eval.unwrap().0)),false))
                        },
                        Opcode::RefMut => {
                            if rt_eval.clone().unwrap().0.is_var_recur() {
                                if is_refmuted(rt_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) 
                                || is_refed(rt_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) {
                                    return Err("Variable already has a mutable reference or reference".to_string());
                                }
                            }
                            Ok((Term::RefMut(Box::new(rt_eval.unwrap().0)),r_b))},
                        Opcode::Deref => {
                            match r_eval {
                                Term::Ref(v) | Term::RefMut(v)=> {
                                    Ok((*v,r_b))
                                },
                                _ => Err("Cannot deref non Ref(Term)".to_string())
                            }
                        },
                        _=> unimplemented!("Implement FuncCall")
                    }
                },
                None => Err("Variable not in scope".to_string())
            }
        },
        Expr::FuncCall(name, params) => {
            let stack_memory = *scope_access;
            let mut func_names = get_func_arg_names(name.clone(), funcs).unwrap();
            let mut scope:HashMap<String,(Term,bool)> = HashMap::new();
            for param in params {
                let param_eval = expr_eval(param, var_env, funcs, scope_access);
                if param_eval.is_err() {
                    return param_eval
                }
                let param_eval = param_eval.unwrap();
                if param_eval.clone().0.is_var() {
                    let var = param_eval.clone().0.get_var().unwrap();
                    let mut param_eval = get_var_val(param_eval.clone().0, var_env, scope_access).unwrap();
                    let mut second_scope = var_env.pop_front().unwrap();
                    second_scope.remove(&var);
                    var_env.push_front(second_scope);
                    //prepend a ! if param is a & or &mut
                    if param_eval.clone().0.is_ref() || param_eval.clone().0.is_refmut() {
                        let mut new_name = param_eval.clone().0.get_var().unwrap();
                        new_name.push_str("!");
                        param_eval = (param_eval.clone().0.change_var_name(new_name),param_eval.1);
                    }
                    scope.insert(func_names.remove(0), param_eval);
                }
                else {
                    let param_eval = get_var_val(param_eval.clone().0, var_env, scope_access);
                    scope.insert(func_names.remove(0), param_eval.unwrap());
                }
            }
            var_env.push_front(scope);
            *scope_access = 1;
            let f = get_func_expr(name, funcs).unwrap();
            let ret = exe_func(f, var_env, funcs, scope_access);
            *scope_access=stack_memory;
            ret
        }
        _ => Err("Not an expr".to_string())
    }
}

pub fn stmnt_eval(expr:Box<Expr>,
    var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
    funcs: &HashMap<String,Box<Expr>>,
    scope_access:&mut usize) -> Result<(Term,bool),String> {
    
    match *expr {
        Expr::Let(m,name,_type,expr) => {
            let r_eval = stmnt_eval(expr, var_env, funcs, scope_access);
            if r_eval.is_err() {
                return r_eval;
            }
            if r_eval.clone().unwrap().0.is_var() {
                if is_refmuted(r_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) 
                || is_refed(r_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) {
                    return Err(format!("Variable {} is already reffed or mutreffed",r_eval.unwrap().0.get_var().unwrap()));
                }
            }
            let r_eval = get_var_val(r_eval.unwrap().0, var_env, scope_access);
            match r_eval {
                Some((r_eval,_r_b)) => {
                    let mut scope = var_env.pop_front().unwrap();
                    scope.insert(name, (r_eval.clone(),m));
                    var_env.push_front(scope);
                    Ok((Term::Unit,m))
                }
                None => Err("Variable not in scope".to_string())
            }
        },
        Expr::Assign(l,r) => {
            let mut check_left = false;
            if l.clone().is_var() {
                check_left = true;
            }
            let r_eval = stmnt_eval(r, var_env, funcs, scope_access);
            let l_eval = stmnt_eval(l, var_env, funcs, scope_access);
            if r_eval.is_err() || l_eval.is_err() {
                return Err(format!("Assign failed, left: {:?} right: {:?}",l_eval,r_eval));
            }
            if r_eval.clone().unwrap().0.is_var() {
                if is_refmuted(r_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) 
                || is_refed(r_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) {
                    return Err(format!("Variable {} is already reffed or mutreffed",r_eval.unwrap().0.get_var().unwrap()));
                }
            }
            if check_left {
                if l_eval.clone().unwrap().0.is_var() {
                    if is_refmuted(l_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) 
                    || is_refed(l_eval.clone().unwrap().0.get_var().unwrap(), var_env, scope_access) {
                        return Err(format!("Variable {} is already reffed or mutreffed",l_eval.unwrap().0.get_var().unwrap()));
                    }
                }
            }
            let r_eval = get_var_val(r_eval.unwrap().0, var_env, scope_access);
            let l_eval = l_eval.unwrap();
            match r_eval {
                Some((r_eval,r_b)) => {
                    if l_eval.1 {
                        let res =insert_assign(l_eval.0.get_var().unwrap(), (r_eval.clone(),l_eval.1),var_env, scope_access);
                        if res.is_err() {return Err("Variable not in scope".to_string());}
                        return Ok((Term::Unit,r_b,));
                    }
                    else {
                        return Err(format!("Assign failed left {:?} isnt mutable",l_eval));
                    }
                },
                None => {return Err(format!("Assign failed right {:?}",r_eval));}
            }
        },
        Expr::If(if_expr,if_block,else_block) => {
            let if_eval = expr_eval(if_expr, var_env, funcs, scope_access);
            if if_eval.is_err() {
                return if_eval;
            }
            let if_eval = get_var_val(if_eval.unwrap().0, var_env, scope_access);
            match if_eval {
                Some(b) => {
                    if b.0.get_bool().unwrap() {
                        *scope_access=*scope_access+1;
                        let scope:HashMap<String,(Term,bool)> = HashMap::new();
                        var_env.push_front(scope);
                        let if_eval = block_eval(if_block, var_env, funcs, scope_access);
                        return if_eval;
                    }
                    match else_block {
                        Some(else_block) => {
                            *scope_access=*scope_access+1;
                            let scope:HashMap<String,(Term,bool)> = HashMap::new();
                            var_env.push_front(scope);
                            let else_eval = block_eval(else_block, var_env, funcs, scope_access);
                            return else_eval;
                        },
                        None => {
                            Ok((Term::Unit,true))
                        }
                    }
                }
                None => Err("Variable not in scope".to_string())
            }
        },
        Expr::While(while_expr,while_block) => {
            let while_eval = expr_eval(while_expr.clone(), var_env, funcs, scope_access);
            if while_eval.is_err() {
                return while_eval;
            }
            let while_eval = get_var_val(while_eval.unwrap().0, var_env, scope_access);
            match while_eval {
                Some(mut while_eval) => {
                    let max_iter = 1000;
                    let mut current_iter = 0;
                    while while_eval.0.get_bool().unwrap() && (current_iter<max_iter) {
                        *scope_access=*scope_access+1;
                        let scope:HashMap<String,(Term,bool)> = HashMap::new();
                        var_env.push_front(scope);
                        let _evaled = block_eval(while_block.clone(), var_env, funcs, scope_access);
                        while_eval = expr_eval(while_expr.clone(), var_env, funcs, scope_access).unwrap();
                        current_iter = current_iter + 1;
                    }
                    Ok((Term::Unit,true))
                },
                None => {
                    Err("Variable not in scope".to_string())
                }
            }

        },
        _ => expr_eval(expr, var_env, funcs, scope_access)
    }
}

fn block_eval(expr_block:Vec<Box<Expr>>,
    var_env: &mut VecDeque<HashMap<String,(Term,bool)>>,
    funcs: &HashMap<String,Box<Expr>>,
    scope_access:&mut usize) -> Result<(Term,bool),String> {

    let last_instr = expr_block.len();
    for expr in expr_block.iter().enumerate() {
        let instr_eval = stmnt_eval(expr.1.clone(), var_env, funcs, scope_access);
        
        if instr_eval.is_err() {
            *scope_access=*scope_access-1;
            var_env.pop_front();
            return instr_eval;
        }
        if expr.0 == last_instr - 1 {
            let mut instr_eval = get_var_val(instr_eval.unwrap().0, var_env, scope_access).unwrap();

            if instr_eval.clone().0.is_ref() || instr_eval.clone().0.is_refmut() {
                let mut name = instr_eval.clone().0.get_var().unwrap();
                let excl = name.pop().unwrap();
                if excl != '!' {
                    return Err("Returned reference lifetime to short".to_string());
                }
                else {
                    instr_eval = (instr_eval.clone().0.change_var_name(name),instr_eval.1);
                }
            }
            *scope_access=*scope_access-1;
            var_env.pop_front();
            return Ok(instr_eval);
        }
    }
    *scope_access=*scope_access-1;
    var_env.pop_front();
    Ok((Term::Unit,true))
}