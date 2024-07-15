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

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Atom(atom) => {
                if atom.arguments.len() == 0 {
                    atom.name.clone()
                } else {
                    let args = atom
                        .arguments
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>();
                    format!("{}({})", atom.name, args.join(", "))
                }
            }
            Expr::Var(var) => {
                format!("${}", var.name)
            }
        }
    }
}
