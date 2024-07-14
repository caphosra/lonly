use crate::ast::Statement;
use crate::env::Environment;
use crate::parser::parse_program;
use evaluation::SolutionGenerator;
use nom_locate::LocatedSpan;

mod ast;
mod env;
mod error;
mod evaluation;
mod parser;

fn main() {
    let program = LocatedSpan::new("num(z) num(s($x)) <- num($x) ?num(s(s(z)))");
    let mut env = Environment::new();
    let (_, statements) = parse_program(program).unwrap();
    for stmt in statements {
        match stmt {
            Statement::Def(stmt) => {
                env.update(stmt).unwrap();
                println!("{:?}", env);
            }
            Statement::Query(mut stmt) => {
                let (mut solution_gen, name_tables) =
                    SolutionGenerator::new(&mut stmt.query, &mut env).unwrap();
                while let Some(solution) = solution_gen.next().unwrap() {
                    let solution = name_tables
                        .iter()
                        .map(|(name, id)| {
                            if let Some(expr) = solution.get(id) {
                                format!("{} = {:?}", name, expr)
                            } else {
                                format!("{} = [missing]", name)
                            }
                        })
                        .collect::<Vec<_>>();
                    println!("[{}]", solution.join(", "));
                }
                println!("No answer remains.");
            }
        }
    }
}
