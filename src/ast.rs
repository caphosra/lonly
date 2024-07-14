use std::collections::HashMap;

pub type VarID = u32;

#[derive(Debug)]
pub struct DefStatement {
    pub conclusion: PredicateObj,
    pub premises: Vec<PredicateObj>,
}

impl PartialEq for DefStatement {
    fn eq(&self, other: &Self) -> bool {
        self.conclusion == other.conclusion || self.premises == other.premises
    }
}

impl DefStatement {
    pub fn new(conclusion: PredicateObj, premises: Vec<PredicateObj>) -> Statement {
        Statement::Def(DefStatement {
            conclusion,
            premises,
        })
    }
}

#[derive(Debug)]
pub struct QueryStatement {
    pub query: PredicateObj,
}

impl PartialEq for QueryStatement {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query
    }
}

impl QueryStatement {
    pub fn new(query: PredicateObj) -> Statement {
        Statement::Query(QueryStatement { query })
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Def(DefStatement),
    Query(QueryStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PredicateObj {
    pub name: String,
    pub arguments: Vec<Expr>,
}

impl PredicateObj {
    pub fn new(name: String, arguments: Vec<Expr>) -> Self {
        PredicateObj { name, arguments }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AtomExpr {
    pub name: String,
    pub arguments: Vec<Expr>,
}

impl AtomExpr {
    pub fn new(name: String, arguments: Vec<Expr>) -> Expr {
        Expr::Atom(AtomExpr { name, arguments })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarExpr {
    pub name: String,
    pub id: Option<VarID>,
}

impl VarExpr {
    pub fn new(name: String) -> Expr {
        Expr::Var(VarExpr { name, id: None })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atom(AtomExpr),
    Var(VarExpr),
}

impl Expr {
    pub fn replace_vars(&mut self, replacement: &HashMap<VarID, Expr>) {
        match self {
            Expr::Atom(atom) => {
                for arg in &mut atom.arguments {
                    arg.replace_vars(replacement);
                }
            }
            Expr::Var(var) => {
                if let Some(id) = var.id {
                    if let Some(expr) = replacement.get(&id) {
                        *self = expr.clone();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replace_var() {
        let x_var = Expr::Var(VarExpr {
            name: "x".to_string(),
            id: Some(0),
        });
        let y_var = Expr::Var(VarExpr {
            name: "y".to_string(),
            id: Some(1),
        });

        let mut expr = AtomExpr::new(
            "add".to_string(),
            vec![x_var, AtomExpr::new("s".to_string(), vec![y_var])],
        );
        let x_expr = AtomExpr::new(
            "s".to_string(),
            vec![AtomExpr::new("z".to_string(), Vec::new())],
        );
        let y_expr = AtomExpr::new("z".to_string(), Vec::new());

        let mut replacement = HashMap::new();
        replacement.insert(0, x_expr.clone());
        replacement.insert(1, y_expr.clone());

        expr.replace_vars(&replacement);
        assert_eq!(
            expr,
            AtomExpr::new(
                "add".to_string(),
                vec![x_expr, AtomExpr::new("s".to_string(), vec![y_expr])]
            )
        );
    }
}
