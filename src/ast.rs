use nom_locate::LocatedSpan;

#[derive(Debug)]
pub struct DefStatement<'a> {
    location: LocatedSpan<&'a str>,
    pub conclusion: Expr<'a>,
    pub premises: Vec<Expr<'a>>,
}

impl<'a> PartialEq for DefStatement<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.conclusion == other.conclusion || self.premises == other.premises
    }
}

impl<'a> DefStatement<'a> {
    pub fn new(
        location: LocatedSpan<&'a str>,
        conclusion: Expr<'a>,
        premises: Vec<Expr<'a>>,
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
    pub query: Expr<'a>,
}

impl<'a> PartialEq for QueryStatement<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.query == other.query
    }
}

impl<'a> QueryStatement<'a> {
    pub fn new(location: LocatedSpan<&'a str>, query: Expr<'a>) -> Statement<'a> {
        Statement::Query(QueryStatement { location, query })
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Def(DefStatement<'a>),
    Query(QueryStatement<'a>),
}

#[derive(Debug, PartialEq)]
pub struct PredicateExpr<'a> {
    pub name: &'a str,
    pub arguments: Vec<Expr<'a>>,
}

impl<'a> PredicateExpr<'a> {
    pub fn new(name: &'a str, arguments: Vec<Expr<'a>>) -> Expr<'a> {
        Expr::Predicate(PredicateExpr { name, arguments })
    }
}

#[derive(Debug, PartialEq)]
pub struct VarExpr {
    pub name: String,
}

impl VarExpr {
    pub fn new<'a>(name: String) -> Expr<'a> {
        Expr::Var(VarExpr { name })
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstExpr {
    pub name: String,
}

impl ConstExpr {
    pub fn new<'a>(name: String) -> Expr<'a> {
        Expr::Const(ConstExpr { name })
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Predicate(PredicateExpr<'a>),
    Var(VarExpr),
    Const(ConstExpr),
}
