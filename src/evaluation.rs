use std::collections::{HashMap, VecDeque};

use crate::{
    ast::{PredicateObj, VarID},
    env::{Environment, VarAllocator, VarSubstitution},
    error::ErrorKind,
    unifier::unify_exprs,
};

pub struct Goals {
    goals: VecDeque<PredicateObj>,
    resolved_vars: VarSubstitution,
}

impl Goals {
    pub fn new(
        goal: &mut PredicateObj,
        var_alloc: &mut VarAllocator,
    ) -> Result<(Self, HashMap<String, u32>), ErrorKind> {
        let mut id_assignments = HashMap::new();
        var_alloc.assign_new_ids(&mut goal.arguments, &mut id_assignments)?;

        let mut goals = VecDeque::new();
        goals.push_back(goal.clone());
        Ok((
            Self {
                goals,
                resolved_vars: VarSubstitution::new(),
            },
            id_assignments,
        ))
    }

    pub fn apply_rule(
        &self,
        var_alloc: &mut VarAllocator,
        conclusion: &PredicateObj,
        premises: &Vec<PredicateObj>,
    ) -> Result<Option<Goals>, ErrorKind> {
        let mut goals = self.goals.clone();
        if let Some(goal) = goals.pop_front() {
            // Copy predicate objects to assign IDs.
            let mut conclusion = conclusion.clone();
            let mut premises: VecDeque<_> = premises.clone().into();
            let mut subst = self.resolved_vars.clone();

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
            let mut exprs = VecDeque::new();
            for idx in 0..goal.arguments.len() {
                exprs.push_back((
                    goal.arguments[idx].clone(),
                    conclusion.arguments[idx].clone(),
                ));
            }
            match unify_exprs(&mut exprs) {
                Ok(new_subst) => {
                    subst.merge(&new_subst);

                    // Replace variables with the solutions.
                    for premise in &mut premises {
                        for arg in &mut premise.arguments {
                            subst.substitute(arg);
                        }
                    }

                    // Replace the first goal with premises.
                    let mut new_goals = premises;
                    new_goals.append(&mut goals);

                    Ok(Some(Goals {
                        goals: new_goals,
                        resolved_vars: subst,
                    }))
                }
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

pub struct SolutionGenerator<'a> {
    status: VecDeque<Goals>,
    env: &'a Environment,
    var_alloc: VarAllocator,
}

impl<'a> SolutionGenerator<'a> {
    pub fn next(&mut self) -> Result<Option<VarSubstitution>, ErrorKind> {
        if let Some(state) = self.status.pop_front() {
            if state.goals.len() == 0 {
                Ok(Some(state.resolved_vars))
            } else {
                if let Some(rules) = self.env.get_rules(&state.goals[0].name) {
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

    pub fn new(
        query: &mut PredicateObj,
        env: &'a Environment,
    ) -> Result<(Self, Vec<(String, VarID)>), ErrorKind> {
        let mut var_alloc = VarAllocator::new();
        let (goal, name_table) = Goals::new(query, &mut var_alloc)?;
        let mut queue = VecDeque::new();
        queue.push_back(goal);
        Ok((
            SolutionGenerator {
                status: queue,
                var_alloc,
                env,
            },
            name_table.into_iter().collect(),
        ))
    }
}
