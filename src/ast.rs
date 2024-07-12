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

#[derive(Debug, PartialEq)]
pub struct AtomExpr<'a> {
    pub name: &'a str,
    pub arguments: Vec<Expr<'a>>,
}

impl<'a> AtomExpr<'a> {
    pub fn new(name: &'a str, arguments: Vec<Expr<'a>>) -> Expr<'a> {
        Expr::Atom(AtomExpr { name, arguments })
    }
}

#[derive(Debug, PartialEq)]
pub struct VarExpr<'a> {
    pub name: &'a str,
    pub id: Option<VarID>,
}

impl<'a> VarExpr<'a> {
    pub fn new(name: &'a str) -> Expr<'a> {
        Expr::Var(VarExpr { name, id: None })
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Atom(AtomExpr<'a>),
    Var(VarExpr<'a>),
}
