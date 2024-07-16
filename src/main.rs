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
                let mut is_interrupted = false;
                while let Some(solution) = solution_gen.next()? {
                    let solution = name_tables
                        .iter()
                        .map(|(name, id)| {
                            let expr = solution.get(*id).unwrap();
                            format!("{} = {}", name, expr.to_string())
                        })
                        .collect::<Vec<_>>();

                    print!("[{}]", solution.join(", "));
                    io::stdout().flush().unwrap();

                    let mut buf = String::new();
                    let stdin = io::stdin();
                    stdin.lock().read_line(&mut buf).unwrap();
                    match buf.as_str() {
                        "\n" => {}
                        ".\n" => {
                            is_interrupted = true;
                            break
                        },
                        _ => Err(ErrorKind::UnknownInstruction)?,
                    }
                }
                if is_interrupted {
                    println!("Interrupted.");
                }
                else {
                    println!("No answer remains.");
                }
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
                ErrorKind::UnknownInstruction => {
                    println!("ERR: This option is not supported.")
                }
            }
        }
    }
}
