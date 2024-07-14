use std::collections::{HashMap, VecDeque};

use crate::{
    ast::{Expr, PredicateObj, VarID},
    env::{Environment, VarAllocator},
    error::ErrorKind,
};

pub struct Goals<'a> {
    goals: VecDeque<PredicateObj<'a>>,
    resolved_vars: HashMap<VarID, Expr<'a>>,
}

impl<'a> Goals<'a> {
    pub fn new(
        goal: &mut PredicateObj<'a>,
        var_alloc: &mut VarAllocator,
    ) -> Result<Self, ErrorKind> {
        let mut id_assignments = HashMap::new();
        var_alloc.assign_new_ids(&mut goal.arguments, &mut id_assignments)?;

        let mut goals = VecDeque::new();
        goals.push_back(goal.clone());
        Ok(Self {
            goals,
            resolved_vars: HashMap::new(),
        })
    }

    pub fn apply_rule(
        &self,
        var_alloc: &mut VarAllocator,
        conclusion: &PredicateObj<'a>,
        premises: &Vec<PredicateObj<'a>>,
    ) -> Result<Option<Goals<'a>>, ErrorKind> {
        let mut goals = self.goals.clone();
        if let Some(goal) = goals.pop_front() {
            // Copy predicate objects to assign IDs.
            let mut conclusion = conclusion.clone();
            let mut premises: VecDeque<_> = premises.clone().into();
            let mut resolved_vars = self.resolved_vars.clone();

            // Assignment new variable IDs.
            let mut id_assignments = HashMap::new();
            var_alloc.assign_new_ids(&mut conclusion.arguments, &mut id_assignments)?;
            for premise in &mut premises {
                var_alloc.assign_new_ids(&mut premise.arguments, &mut id_assignments)?;
            }

            if goal.arguments.len() != conclusion.arguments.len() {
                return Ok(None);
            }

            // Applying the rule by unifying variables.
            for idx in 0..goal.arguments.len() {
                if let Err(_) = unify_exprs(
                    &goal.arguments[idx],
                    &conclusion.arguments[idx],
                    &mut resolved_vars,
                ) {
                    return Ok(None);
                }
            }

            // Replace variables with the solutions.
            for premise in &mut premises {
                for arg in &mut premise.arguments {
                    arg.replace_vars(&resolved_vars);
                }
            }

            // Replace the first goal with premises.
            let mut new_goals = premises;
            new_goals.append(&mut goals);

            Ok(Some(Goals {
                goals: new_goals,
                resolved_vars,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct SolutionGenerator<'a, 'b> {
    status: VecDeque<Goals<'a>>,
    env: &'b Environment<'a>,
    var_alloc: VarAllocator,
}

impl<'a, 'b> SolutionGenerator<'a, 'b> {
    pub fn next(&mut self) -> Result<Option<HashMap<VarID, Expr<'a>>>, ErrorKind> {
        if let Some(state) = self.status.pop_front() {
            if state.goals.len() == 0 {
                Ok(Some(state.resolved_vars))
            } else {
                if let Some(rules) = self.env.get_rules(state.goals[0].name) {
                    for (conclusion, premises) in rules {
                        let new_goals =
                            state.apply_rule(&mut self.var_alloc, conclusion, premises)?;
                        if let Some(new_goals) = new_goals {
                            self.status.push_back(new_goals);
                        }
                    }
                }
                self.next()
            }
        } else {
            Ok(None)
        }
    }

    pub fn new(goal: &mut PredicateObj<'a>, env: &'b Environment<'a>) -> Result<Self, ErrorKind> {
        let mut var_alloc = VarAllocator::new();
        let goal = Goals::new(goal, &mut var_alloc)?;
        let mut queue = VecDeque::new();
        queue.push_back(goal);
        Ok(SolutionGenerator {
            status: queue,
            var_alloc: var_alloc,
            env,
        })
    }
}

fn occur_check<'a>(expr: &Expr<'a>, id: VarID) -> bool {
    match expr {
        Expr::Atom(atom) => atom.arguments.iter().any(|expr| occur_check(expr, id)),
        Expr::Var(v) => Some(id) == v.id,
    }
}

pub fn unify_exprs<'a, 'b>(
    expr1: &'b Expr<'a>,
    expr2: &'b Expr<'a>,
    constraints: &mut HashMap<VarID, Expr<'a>>,
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

    use super::*;

    #[test]
    fn test_unify1() {
        let expr1 = AtomExpr::new("s", vec![VarExpr::new("t")]);
        let expr2 = AtomExpr::new("s", vec![AtomExpr::new("a", Vec::new())]);

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
        let expr1 = AtomExpr::new("s", vec![VarExpr::new("t")]);
        let expr2 = AtomExpr::new("s", vec![AtomExpr::new("s", vec![VarExpr::new("t")])]);

        let mut var_alloc = VarAllocator::new();
        let mut exprs = vec![expr1, expr2];
        let mut assigned = HashMap::new();
        assert!(var_alloc.assign_new_ids(&mut exprs, &mut assigned).is_ok());

        let mut constraints = HashMap::new();
        assert!(unify_exprs(&exprs[0], &exprs[1], &mut constraints).is_err());
    }
}
