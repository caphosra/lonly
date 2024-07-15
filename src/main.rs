use crate::ast::Statement;
use crate::env::Environment;
use crate::parser::parse_program;
use error::ErrorKind;
use evaluation::SolutionGenerator;
use nom_locate::LocatedSpan;
use std::io::{self, BufRead, Write};

mod ast;
mod env;
mod error;
mod evaluation;
mod parser;
mod unifier;

fn exec_program(env: &mut Environment, program: &str) -> Result<(), ErrorKind> {
    let program = LocatedSpan::new(program);
    let (_, statements) = parse_program(program)?;
    for stmt in statements {
        match stmt {
            Statement::Def(stmt) => {
                env.update(stmt)?;
            }
            Statement::Query(mut stmt) => {
                let (mut solution_gen, name_tables) = SolutionGenerator::new(&mut stmt.query, env)?;
                while let Some(solution) = solution_gen.next()? {
                    let solution = name_tables
                        .iter()
                        .map(|(name, id)| {
                            if let Some(expr) = solution.get(*id) {
                                format!("{} = {}", name, expr.to_string())
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
    Ok(())
}

fn main() {
    let mut env = Environment::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line).unwrap();
        if let Err(err) = exec_program(&mut env, &line) {
            match err {
                ErrorKind::ArityMismatch(name, size1, size2) => {
                    println!(
                        "ERR: Arity of the predicate \"{}\" is expected to be {}, but is {}.",
                        name, size1, size2
                    );
                }
                ErrorKind::VariableIDAlreadyAssigned(name) => {
                    println!("ERR: The id of variable \"{}\" is already assigned.", name);
                }
                ErrorKind::Parser(text) => {
                    println!(
                        "ERR: An error detected while parsing program. Detail: {}",
                        text
                    );
                }
            }
        }
    }
}
