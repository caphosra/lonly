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

use crate::ast::{
    ConstExpr, DefStatement, Expr, PredicateExpr, QueryStatement, Statement, VarExpr,
};

type ParseResult<'a, T> = IResult<LocatedSpan<&'a str>, T, VerboseError<LocatedSpan<&'a str>>>;

pub fn parse_program<'a>(program: LocatedSpan<&'a str>) -> ParseResult<Vec<Statement<'a>>> {
    delimited(
        multispace0,
        separated_list0(multispace1, parse_statement),
        tuple((multispace0, eof)),
    )(program)
}

fn parse_statement<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Statement<'a>> {
    alt((parse_query_statement, parse_def_statement))(text)
}

fn parse_query_statement<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Statement<'a>> {
    let (text, (_, _, query)) = tuple((tag("?"), multispace0, parse_predicate))(text)?;
    Ok((text, QueryStatement::new(text, query)))
}

fn parse_def_statement<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Statement<'a>> {
    let (text, (conclusion, premises)) = tuple((
        parse_predicate,
        opt(tuple((
            multispace0,
            tag("<-"),
            multispace0,
            separated_list1(tuple((multispace0, tag(","), multispace0)), parse_expr),
        ))),
    ))(text)?;

    let premises = premises.map_or(vec![], |(_, _, _, premises)| premises);
    Ok((text, DefStatement::new(text, conclusion, premises)))
}

fn is_alphanumeric_or_underscore(s: char) -> bool {
    s.is_ascii_alphanumeric() || s == '_'
}

fn parse_expr<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr<'a>> {
    alt((parse_predicate, parse_var, parse_const))(text)
}

fn parse_predicate<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr<'a>> {
    let (text, ident) = take_while1(is_alphanumeric_or_underscore)(text)?;
    let (text, l) = delimited(
        tuple((tag("("), multispace0)),
        separated_list1(tuple((multispace0, tag(","), multispace0)), parse_expr),
        tuple((multispace0, tag(")"))),
    )(text)?;
    Ok((text, PredicateExpr::new(&ident, l)))
}

fn parse_var<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr<'a>> {
    let (text, _) = tag("$")(text)?;
    let (text, ident) = take_while1(is_alphanumeric_or_underscore)(text)?;
    Ok((text, VarExpr::new(ident.to_string())))
}

fn parse_const<'a>(text: LocatedSpan<&'a str>) -> ParseResult<Expr<'a>> {
    let (text, ident) = take_while1(is_alphanumeric_or_underscore)(text)?;
    Ok((text, ConstExpr::new(ident.to_string())))
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
            QueryStatement::new(
                LocatedSpan::new(""),
                Expr::Predicate(PredicateExpr {
                    name: "test_1dent",
                    arguments: vec![
                        Expr::Var(VarExpr {
                            name: "x".to_string()
                        }),
                        Expr::Const(ConstExpr {
                            name: "z".to_string()
                        }),
                        Expr::Predicate(PredicateExpr {
                            name: "s",
                            arguments: vec![Expr::Var(VarExpr {
                                name: "y".to_string()
                            })]
                        })
                    ]
                })
            )
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
            Expr::Predicate(PredicateExpr {
                name: "test_1dent",
                arguments: vec![
                    Expr::Predicate(PredicateExpr {
                        name: "s",
                        arguments: vec![Expr::Var(VarExpr {
                            name: "x".to_string()
                        })]
                    }),
                    Expr::Var(VarExpr {
                        name: "x".to_string()
                    })
                ]
            })
        );
    }
}
