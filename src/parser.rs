use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{multispace0, multispace1},
    combinator::{eof, opt},
    error::VerboseError,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};
use nom_locate::LocatedSpan;

use crate::ast::{AtomExpr, DefStatement, Expr, PredicateObj, QueryStatement, Statement, VarExpr};

type ParseResult<'a, T> = IResult<LocatedSpan<&'a str>, T, VerboseError<LocatedSpan<&'a str>>>;

pub fn parse_program<'a>(program: LocatedSpan<&'a str>) -> ParseResult<Vec<Statement>> {
    delimited(
        multispace0,
        separated_list0(multispace1, parse_statement),
        tuple((multispace0, eof)),
    )(program)
}

fn parse_statement<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Statement> {
    alt((parse_query_statement, parse_def_statement))(text)
}

fn parse_query_statement<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Statement> {
    let (text, (_, _, query)) = tuple((tag("?"), multispace0, parse_predicate))(text)?;
    Ok((text, QueryStatement::new(query)))
}

fn parse_def_statement<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Statement> {
    let (text, (conclusion, premises)) = tuple((
        parse_predicate,
        opt(tuple((
            multispace0,
            tag("<-"),
            multispace0,
            separated_list1(tuple((multispace0, tag(","), multispace0)), parse_predicate),
        ))),
    ))(text)?;

    let premises = premises.map_or(vec![], |(_, _, _, premises)| premises);
    Ok((text, DefStatement::new(conclusion, premises)))
}

fn is_alphanumeric_or_underscore(s: char) -> bool {
    s.is_ascii_alphanumeric() || s == '_'
}

fn parse_ident<'a>(text: LocatedSpan<&'a str>) -> ParseResult<LocatedSpan<&'a str>> {
    take_while1(is_alphanumeric_or_underscore)(text)
}

fn parse_n_ary<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Vec<Expr>> {
    delimited(
        tuple((tag("("), multispace0)),
        separated_list1(tuple((multispace0, tag(","), multispace0)), parse_expr),
        tuple((multispace0, tag(")"))),
    )(text)
}

fn parse_predicate<'a>(text: LocatedSpan<&'a str>) -> ParseResult<PredicateObj> {
    let (text, ident) = parse_ident(text)?;
    let (text, l) = parse_n_ary(text)?;
    Ok((text, PredicateObj::new(ident.to_string(), l)))
}

fn parse_expr<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr> {
    alt((parse_var, parse_n_ary_atom, parse_nullary_atom))(text)
}

fn parse_var<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr> {
    let (text, _) = tag("$")(text)?;
    let (text, ident) = parse_ident(text)?;
    Ok((text, VarExpr::new(ident.to_string())))
}

fn parse_n_ary_atom<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr> {
    let (text, ident) = parse_ident(text)?;
    let (text, l) = parse_n_ary(text)?;
    Ok((text, AtomExpr::new(ident.to_string(), l)))
}

fn parse_nullary_atom<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr> {
    let (text, ident) = parse_ident(text)?;
    Ok((text, AtomExpr::new(ident.to_string(), vec![])))
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn parse_statement_test1() {
        let parsed = parse_statement(LocatedSpan::new("?test_1dent($x, z, s($y))"));
        assert!(parsed.is_ok());
        let (text, item) = parsed.unwrap();
        assert_eq!(text.to_string(), "");
        assert_eq!(
            item,
            QueryStatement::new(PredicateObj::new(
                "test_1dent".to_string(),
                vec![
                    VarExpr::new("x".to_string()),
                    AtomExpr::new("z".to_string(), Vec::new()),
                    AtomExpr::new("s".to_string(), vec![VarExpr::new("y".to_string())])
                ]
            ))
        );
    }

    #[test]
    fn parse_expr_test() {
        let parsed = parse_expr(LocatedSpan::new("test_1dent( s($x), $x)remains"));
        assert!(parsed.is_ok());
        let (text, item) = parsed.unwrap();
        assert_eq!(text.to_string(), "remains");
        assert_eq!(
            item,
            AtomExpr::new(
                "test_1dent".to_string(),
                vec![
                    AtomExpr::new("s".to_string(), vec![VarExpr::new("x".to_string())]),
                    VarExpr::new("x".to_string())
                ]
            )
        );
    }
}
