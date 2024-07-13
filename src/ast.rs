use nom_locate::LocatedSpan;

pub type VarID = u32;

#[derive(Debug)]
pub struct DefStatement<'a> {
    location: LocatedSpan<&'a str>,
    pub conclusion: PredicateObj<'a>,
    pub premises: Vec<PredicateObj<'a>>,
}

impl<'a> PartialEq for DefStatement<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.conclusion == other.conclusion || self.premises == other.premises
    }
}

impl<'a> DefStatement<'a> {
    pub fn new(
        location: LocatedSpan<&'a str>,
        conclusion: PredicateObj<'a>,
        premises: Vec<PredicateObj<'a>>,
    ) -> Statement<'a> {
        Statement::Def(DefStatement {
            location,
            conclusion,
            premises,
        })
    }
}

#[derive(Debug)]
pub struct QueryStatement<'a> {
    location: LocatedSpan<&'a str>,
    pub query: PredicateObj<'a>,
}

impl<'a> PartialEq for QueryStatement<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query
    }
}

impl<'a> QueryStatement<'a> {
    pub fn new(location: LocatedSpan<&'a str>, query: PredicateObj<'a>) -> Statement<'a> {
        Statement::Query(QueryStatement { location, query })
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Def(DefStatement<'a>),
    Query(QueryStatement<'a>),
}

#[derive(Debug, PartialEq)]
pub struct PredicateObj<'a> {
    pub name: &'a str,
    pub arguments: Vec<Expr<'a>>,
}

impl<'a> PredicateObj<'a> {
    pub fn new(name: &'a str, arguments: Vec<Expr<'a>>) -> Self {
        PredicateObj { name, arguments }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AtomExpr<'a> {
    pub name: &'a str,
    pub arguments: Vec<Expr<'a>>,
}

impl<'a> AtomExpr<'a> {
    pub fn new(name: &'a str, arguments: Vec<Expr<'a>>) -> Expr<'a> {
        Expr::Atom(AtomExpr { name, arguments })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarExpr<'a> {
    pub name: &'a str,
    pub id: Option<VarID>,
}

impl<'a> VarExpr<'a> {
    pub fn new(name: &'a str) -> Expr<'a> {
        Expr::Var(VarExpr { name, id: None })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Atom(AtomExpr<'a>),
    Var(VarExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn replace_var<'b>(&mut self, id: VarID, expr: &'b Expr<'a>) {
        match self {
            Expr::Atom(atom) => {
                for arg in &mut atom.arguments {
                    arg.replace_var(id, expr);
                }
            }
            Expr::Var(var) => {
                if var.id == Some(id) {
                    *self = expr.clone();
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
            name: "x",
            id: Some(0),
        });
        let y_var = Expr::Var(VarExpr {
            name: "y",
            id: Some(1),
        });
        let mut expr = AtomExpr::new("add", vec![x_var, AtomExpr::new("s", vec![y_var])]);
        let x_expr = AtomExpr::new("s", vec![AtomExpr::new("z", Vec::new())]);
        let y_expr = AtomExpr::new("z", Vec::new());
        expr.replace_var(0, &x_expr);
        expr.replace_var(1, &y_expr);
        assert_eq!(
            expr,
            AtomExpr::new("add", vec![x_expr, AtomExpr::new("s", vec![y_expr])])
        );
    }
}
