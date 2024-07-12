use std::collections::HashMap;

use crate::ast::{DefStatement, Expr, PredicateObj, VarID};

struct Predicate<'a> {
    length: usize,
    rules: Vec<(PredicateObj<'a>, Vec<PredicateObj<'a>>)>,
}

impl<'a> Predicate<'a> {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            rules: Vec::new(),
        }
    }
}

pub struct Environment<'a> {
    predicates: HashMap<String, Predicate<'a>>,
    num_vars: u32,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            predicates: HashMap::new(),
            num_vars: 0,
        }
    }

    fn gen_new_id(&mut self) -> VarID {
        let id = self.num_vars;
        self.num_vars += 1;
        id
    }

    pub fn assign_new_ids(
        &mut self,
        exprs: &mut Vec<Expr<'a>>,
        assigned: &mut HashMap<String, u32>,
    ) -> Result<(), ()> {
        for expr in exprs {
            match expr {
                Expr::Var(var) => {
                    if var.id == None {
                        if let Some(id) = assigned.get(var.name) {
                            var.id = Some(*id);
                        } else {
                            let id = self.gen_new_id();
                            assigned.insert(var.name.to_string(), id);
                            var.id = Some(id);
                        }
                    } else {
                        Err(())?
                    }
                }
                Expr::Atom(atom) => {
                    self.assign_new_ids(&mut atom.arguments, assigned)?;
                }
            }
        }
        Ok(())
    }

    pub fn validate(&mut self, pred_obj: &PredicateObj<'a>) -> Result<(), ()> {
        let arg_len = pred_obj.arguments.len();
        match self.predicates.get_mut(pred_obj.name) {
            Some(pred) => {
                if arg_len != pred.length {
                    Err(())
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

    pub fn update(&mut self, mut stmt: DefStatement<'a>) -> Result<(), ()> {
        // Validate premises.
        for premise in &stmt.premises {
            self.validate(premise)?;
        }

        // Assign IDs to the variables.
        let mut assigned = HashMap::new();
        self.assign_new_ids(&mut stmt.conclusion.arguments, &mut assigned)?;
        for premise in &mut stmt.premises {
            self.assign_new_ids(&mut premise.arguments, &mut assigned)?;
        }

        let conclusion_len = stmt.conclusion.arguments.len();
        match self.predicates.get_mut(stmt.conclusion.name) {
            Some(pred) => {
                if conclusion_len != pred.length {
                    Err(())
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
