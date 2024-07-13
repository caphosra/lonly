#[derive(Debug)]
pub enum ErrorKind {
    ArityMismatch(String, usize, usize),
    VariableIDAlreadyAssigned(String),
}
