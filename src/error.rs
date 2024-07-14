use nom::error::VerboseError;
use nom_locate::LocatedSpan;

#[derive(Debug)]
pub enum ErrorKind {
    Parser(String),
    ArityMismatch(String, usize, usize),
    VariableIDAlreadyAssigned(String),
}

type NomErr<'a> = nom::Err<VerboseError<LocatedSpan<&'a str>>>;

impl<'a> From<NomErr<'a>> for ErrorKind {
    fn from(value: NomErr<'a>) -> Self {
        ErrorKind::Parser(value.to_string())
    }
}
