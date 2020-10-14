use crate::ast::*;

pub fn expr_eval(expr:Box<Expr>) -> Result<Term,String> {
    match *expr {
        Expr::Boolean(b) => Ok(Term::Bool(b)),
        Expr::Number(i) => Ok(Term::Num(i)),
        Expr::Infix(l,op,r) => {
            let l_eval = expr_eval(l);
            let r_eval = expr_eval(r);
            if l_eval.is_err() || r_eval.is_err() {
                return Err("Infix error".to_string());
            }
            let l_eval = l_eval.unwrap();
            let r_eval = r_eval.unwrap();
            match op {
                Opcode::Add => {
                    Ok(Term::Num(l_eval.get_num().unwrap() + r_eval.get_num().unwrap()))
                },
                Opcode::Sub => {
                    Ok(Term::Num(l_eval.get_num().unwrap() - r_eval.get_num().unwrap()))
                },
                Opcode::Mul => {
                    Ok(Term::Num(l_eval.get_num().unwrap() * r_eval.get_num().unwrap()))
                },
                Opcode::Div => {
                    Ok(Term::Num(l_eval.get_num().unwrap() / r_eval.get_num().unwrap()))
                },
                Opcode::Less => {
                    Ok(Term::Bool(l_eval.get_num().unwrap() < r_eval.get_num().unwrap()))
                },
                Opcode::Greater => {
                    Ok(Term::Bool(l_eval.get_num().unwrap() > r_eval.get_num().unwrap()))
                },
                Opcode::And => {
                    Ok(Term::Bool(l_eval.get_bool().unwrap() && r_eval.get_bool().unwrap()))
                },
                Opcode::Or => {
                    Ok(Term::Bool(l_eval.get_bool().unwrap() || r_eval.get_bool().unwrap()))
                },
                Opcode::Equals => {
                    match l_eval {
                        Term::Num(l) => {
                            Ok(Term::Bool(l == r_eval.get_num().unwrap()))
                        },
                        Term::Bool(l) => {
                            Ok(Term::Bool(l == r_eval.get_bool().unwrap()))
                        },
                        _ => Err("Type not applicable for Equals operation".to_string())
                    }
                },
                _ => Err("Opcode isnt infix".to_string())
            }
        },
        Expr::Prefix(op, r) => {
            let r_eval = expr_eval(r);
            if r_eval.is_err() {
                return r_eval
            }
            let r_eval = r_eval.unwrap();
            match op {
                Opcode::Negate => Ok(Term::Num(-r_eval.get_num().unwrap())),
                Opcode::Not => Ok(Term::Bool(!r_eval.get_bool().unwrap())),
                _ => Err("Opcode isnt Prefix type".to_string())
            }
        },
        Expr::Unary(op,r) => {
            let r_eval = expr_eval(r);
            if r_eval.is_err() {
                return r_eval
            }
            let r_eval = r_eval.unwrap();
            match op {
                Opcode::Ref => Ok(Term::Ref(Box::new(r_eval))),
                Opcode::RefMut => Ok(Term::RefMut(Box::new(r_eval))),
                Opcode::Deref => {
                    if r_eval.is_ref() {
                        Ok(r_eval.pop().unwrap())
                    }
                    else if r_eval.is_refmut() {
                        Ok(r_eval.pop().unwrap())
                    }
                    else {
                        Err("Cannot deref non Ref(Term)".to_string())
                    }
                },
                _=> unimplemented!("Implement FuncCall")
            }
        },
        _ => unimplemented!("Implement FuncCall")
    }
}

fn block_eval() -> () {
    unimplemented!();
}