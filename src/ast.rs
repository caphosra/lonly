#[derive(Debug, PartialEq)]
pub struct DefStatement<'a> {
    pub conclusion: Expr<'a>,
    pub premises: Vec<Expr<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct QueryStatement<'a> {
    pub query: Expr<'a>,
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

#[derive(Debug, PartialEq)]
pub struct VarExpr {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct ConstExpr {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Predicate(PredicateExpr<'a>),
    Var(VarExpr),
    Const(ConstExpr),
}
