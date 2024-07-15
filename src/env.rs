use std::collections::HashMap;

use crate::{
    ast::{DefStatement, Expr, PredicateObj, VarID},
    error::ErrorKind,
};

#[derive(Debug)]
struct Predicate {
    length: usize,
    pub rules: Vec<(PredicateObj, Vec<PredicateObj>)>,
}

impl Predicate {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            rules: Vec::new(),
        }
    }
}

pub struct VarAllocator {
    num_vars: u32,
}

impl VarAllocator {
    pub fn new() -> Self {
        Self { num_vars: 0 }
    }

    fn gen_new_id(&mut self) -> VarID {
        let id = self.num_vars;
        self.num_vars += 1;
        id
    }

    pub fn assign_new_ids(
        &mut self,
        exprs: &mut Vec<Expr>,
        assigned: &mut HashMap<String, u32>,
    ) -> Result<(), ErrorKind> {
        for expr in exprs {
            match expr {
                Expr::Var(var) => {
                    if var.id == None {
                        if let Some(id) = assigned.get(&var.name) {
                            var.id = Some(*id);
                        } else {
                            let id = self.gen_new_id();
                            assigned.insert(var.name.to_string(), id);
                            var.id = Some(id);
                        }
                    } else {
                        Err(ErrorKind::VariableIDAlreadyAssigned(var.name.to_string()))?
                    }
                }
                Expr::Atom(atom) => {
                    self.assign_new_ids(&mut atom.arguments, assigned)?;
                }
            }
        }
        Ok(())
    }
}

///
/// Represents the substitution of variables.
///
#[derive(Clone)]
pub struct VarSubstitution {
    subst: HashMap<VarID, Expr>,
}

impl VarSubstitution {
    pub fn new() -> Self {
        VarSubstitution {
            subst: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: VarID, expr: Expr) {
        self.subst.insert(id, expr);
    }

    pub fn get(&self, id: VarID) -> Option<&Expr> {
        self.subst.get(&id)
    }

    pub fn substitute(&self, expr: &mut Expr) {
        match expr {
            Expr::Atom(atom) => {
                for arg in &mut atom.arguments {
                    self.substitute(arg);
                }
            }
            Expr::Var(var) => {
                if let Some(id) = var.id {
                    if let Some(repl) = self.get(id) {
                        *expr = repl.clone();
                    }
                }
            }
        }
    }

    pub fn merge(&mut self, other: &Self) {
        let ids = self.subst.keys().cloned().collect::<Vec<_>>();
        for id in ids {
            let to_be_replaced = self.subst.get_mut(&id).unwrap();
            other.substitute(to_be_replaced);
        }

        for (id, expr) in &other.subst {
            if !self.subst.contains_key(id) {
                self.subst.insert(*id, expr.clone());
            }
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    predicates: HashMap<String, Predicate>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            predicates: HashMap::new(),
        }
    }

    pub fn get_rules(&self, name: &str) -> Option<&Vec<(PredicateObj, Vec<PredicateObj>)>> {
        self.predicates.get(name).map(|pred| &pred.rules)
    }

    pub fn validate(&mut self, pred_obj: &PredicateObj) -> Result<(), ErrorKind> {
        let arg_len = pred_obj.arguments.len();
        match self.predicates.get_mut(&pred_obj.name) {
            Some(pred) => {
                if arg_len != pred.length {
                    Err(ErrorKind::ArityMismatch(
                        pred_obj.name.to_string(),
                        arg_len,
                        pred.length,
                    ))
                } else {
                    Ok(())
                }
            }
            None => {
                self.predicates
                    .insert(pred_obj.name.to_string(), Predicate::new(arg_len));
                Ok(())
            }
        }
    }

    pub fn update(&mut self, stmt: DefStatement) -> Result<(), ErrorKind> {
        // Validate premises.
        for premise in &stmt.premises {
            self.validate(premise)?;
        }

        let conclusion_len = stmt.conclusion.arguments.len();
        match self.predicates.get_mut(&stmt.conclusion.name) {
            Some(pred) => {
                if conclusion_len != pred.length {
                    Err(ErrorKind::ArityMismatch(
                        stmt.conclusion.name.to_string(),
                        pred.length,
                        conclusion_len,
                    ))
                } else {
                    pred.rules.push((stmt.conclusion, stmt.premises));
                    Ok(())
                }
            }
            None => {
                let mut pred = Predicate::new(conclusion_len);
                let name = stmt.conclusion.name.to_string();
                pred.rules.push((stmt.conclusion, stmt.premises));
                self.predicates.insert(name, pred);
                Ok(())
            }
        }
    }
}
