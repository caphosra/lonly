use std::collections::VecDeque;

use crate::{
    ast::{Expr, VarID},
    env::VarSubstitution,
};

fn var_occurs(expr: &Expr, id: VarID) -> bool {
    match expr {
        Expr::Atom(atom) => atom.arguments.iter().any(|expr| var_occurs(expr, id)),
        Expr::Var(v) => Some(id) == v.id,
    }
}

pub fn unify_exprs(exprs: &mut VecDeque<(Expr, Expr)>) -> Result<VarSubstitution, ()> {
    if let Some((expr1, expr2)) = exprs.pop_front() {
        match (&expr1, &expr2) {
            (Expr::Atom(atom1), Expr::Atom(atom2)) => {
                if atom1.name != atom2.name {
                    Err(())?
                }
                if atom1.arguments.len() != atom2.arguments.len() {
                    Err(())?
                }
                for idx in 0..atom1.arguments.len() {
                    exprs.push_front((atom1.arguments[idx].clone(), atom2.arguments[idx].clone()));
                }
                unify_exprs(exprs)
            }
            (Expr::Var(var1), Expr::Atom(_)) => {
                let id = var1.id.ok_or(())?;
                if var_occurs(&expr2, id) {
                    Err(())
                } else {
                    let mut subst = VarSubstitution::new();
                    subst.insert(id, expr2.clone());

                    let mut exprs = exprs
                        .iter()
                        .map(|(expr1, expr2)| {
                            let mut expr1 = expr1.clone();
                            let mut expr2 = expr2.clone();
                            subst.substitute(&mut expr1);
                            subst.substitute(&mut expr2);
                            (expr1, expr2)
                        })
                        .collect::<VecDeque<_>>();

                    let res = unify_exprs(&mut exprs)?;
                    subst.merge(&res);
                    Ok(subst)
                }
            }
            (Expr::Atom(_), Expr::Var(_)) => {
                exprs.push_front((expr2, expr1));
                unify_exprs(exprs)
            }
            (Expr::Var(var1), Expr::Var(var2)) => {
                let id1 = var1.id.ok_or(())?;
                let id2 = var2.id.ok_or(())?;
                if id1 == id2 {
                    unify_exprs(exprs)
                } else {
                    let mut subst = VarSubstitution::new();
                    subst.insert(id1, expr2.clone());

                    let mut exprs = exprs
                        .iter()
                        .map(|(expr1, expr2)| {
                            let mut expr1 = expr1.clone();
                            let mut expr2 = expr2.clone();
                            subst.substitute(&mut expr1);
                            subst.substitute(&mut expr2);
                            (expr1, expr2)
                        })
                        .collect::<VecDeque<_>>();

                    let res = unify_exprs(&mut exprs)?;
                    subst.merge(&res);
                    Ok(subst)
                }
            }
        }
    } else {
        Ok(VarSubstitution::new())
    }
}
