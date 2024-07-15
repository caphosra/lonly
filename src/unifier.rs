use std::collections::HashMap;

use crate::ast::{Expr, VarID};

fn occur_check(expr: &Expr, id: VarID) -> bool {
    match expr {
        Expr::Atom(atom) => atom.arguments.iter().any(|expr| occur_check(expr, id)),
        Expr::Var(v) => Some(id) == v.id,
    }
}

pub fn unify_exprs<'a, 'b>(
    expr1: &'b Expr,
    expr2: &'b Expr,
    constraints: &mut HashMap<VarID, Expr>,
) -> Result<(), ()> {
    match (expr1, expr2) {
        (Expr::Atom(atom1), Expr::Atom(atom2)) => {
            if atom1.name != atom2.name {
                Err(())?
            }
            if atom1.arguments.len() != atom2.arguments.len() {
                Err(())?
            }
            for idx in 0..atom1.arguments.len() {
                unify_exprs(&atom1.arguments[idx], &atom2.arguments[idx], constraints)?;
            }
            Ok(())
        }
        (Expr::Var(var1), Expr::Atom(_)) => {
            let id = var1.id.ok_or(())?;
            if let Some(exp1) = constraints.get(&id) {
                unify_exprs(&exp1.clone(), expr2, constraints)
            } else {
                if occur_check(expr2, id) {
                    Err(())
                } else {
                    constraints.insert(id, expr2.clone());
                    Ok(())
                }
            }
        }
        (Expr::Atom(_), Expr::Var(_)) => unify_exprs(expr2, expr1, constraints),
        (Expr::Var(var1), Expr::Var(var2)) => {
            let id1 = var1.id.ok_or(())?;
            if let Some(exp1) = constraints.get(&id1) {
                unify_exprs(&exp1.clone(), expr2, constraints)
            } else {
                let id2 = var2.id.ok_or(())?;
                if let Some(exp2) = constraints.get(&id2) {
                    unify_exprs(expr1, &exp2.clone(), constraints)
                } else {
                    constraints.insert(id1, expr2.clone());
                    Ok(())
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{AtomExpr, VarExpr};
    use crate::env::VarAllocator;

    use super::*;

    #[test]
    fn test_unify1() {
        let expr1 = AtomExpr::new("s".to_string(), vec![VarExpr::new("t".to_string())]);
        let expr2 = AtomExpr::new(
            "s".to_string(),
            vec![AtomExpr::new("a".to_string(), Vec::new())],
        );

        let mut var_alloc = VarAllocator::new();
        let mut exprs = vec![expr1, expr2];
        let mut assigned = HashMap::new();
        assert!(var_alloc.assign_new_ids(&mut exprs, &mut assigned).is_ok());

        let mut constraints = HashMap::new();
        assert!(unify_exprs(&exprs[0], &exprs[1], &mut constraints).is_ok());

        assert!(constraints.get(&0).is_some());
    }

    #[test]
    fn test_unify2() {
        let expr1 = AtomExpr::new("s".to_string(), vec![VarExpr::new("t".to_string())]);
        let expr2 = AtomExpr::new(
            "s".to_string(),
            vec![AtomExpr::new(
                "s".to_string(),
                vec![VarExpr::new("t".to_string())],
            )],
        );

        let mut var_alloc = VarAllocator::new();
        let mut exprs = vec![expr1, expr2];
        let mut assigned = HashMap::new();
        assert!(var_alloc.assign_new_ids(&mut exprs, &mut assigned).is_ok());

        let mut constraints = HashMap::new();
        assert!(unify_exprs(&exprs[0], &exprs[1], &mut constraints).is_err());
    }
}
