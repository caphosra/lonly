use std::collections::HashMap;

use crate::ast::{DefStatement, PredicateObj};

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
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self { predicates: HashMap::new() }
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

    pub fn update(&mut self, stmt: DefStatement<'a>) -> Result<(), ()> {
        for premise in &stmt.premises {
            self.validate(premise)?;
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
