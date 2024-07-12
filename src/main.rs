use crate::parser::parse_program;
use crate::ast::Statement;
use crate::env::Environment;
use nom_locate::LocatedSpan;

mod ast;
mod env;
mod parser;

fn main() {
    let program = LocatedSpan::new("num(z) num(s($x)) <- num($x) ?num(s(s(z)))");
    let mut env = Environment::new();
    let (_, statements) = parse_program(program).unwrap();
    for stmt in statements {
        match stmt {
            Statement::Def(stmt) => {
                env.update(stmt).unwrap();
            }
            _ => { }
        }
    }
}
