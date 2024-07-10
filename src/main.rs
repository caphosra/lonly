use crate::parser::parse_program;
use nom_locate::LocatedSpan;

mod ast;
mod parser;

fn main() {
    let program = LocatedSpan::new("num(z) num(s($x)) <- num($x) ?num(s(s(z)))");
    let (_, ast) = parse_program(program).unwrap();
    println!("{:?}", ast);
}
