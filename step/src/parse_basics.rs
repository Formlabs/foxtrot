use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, alphanumeric1, multispace0, multispace1, one_of},
    combinator::{opt, recognize},
    error::{context, VerboseError},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use std::str;
use crate::ap214_autogen::Id;

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn step_decimal(input: &str) -> Res<&str, i64> {
    context(
        "decimal",
        tuple((opt(one_of("-+")), recognize(many1(one_of("0123456789"))))),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (opt_sign, digits) = res;
            let sign = match opt_sign {
                None => 1,
                Some(c) => match c {
                    '+' => 1,
                    '-' => -1,
                    _ => panic!("unexpected char"),
                },
            };
            sign * digits.parse::<i64>().unwrap()
        })
    })
}
pub fn step_float(input: &str) -> Res<&str, f64> {
    context(
        "float",
        tuple((
            opt(tag("-")),
            recognize(
                tuple((
                    many1(one_of("0123456789")),
                    tag("."),
                    many0(one_of("0123456789"))
                ))
            ),
            opt(preceded(tag("E"), step_decimal)),
        )),
    )(input)
    .map(|(next_input, res)| {
        (next_input, {
            let (opt_sign, num, opt_exp) = res;
            let sign = match opt_sign {
                Some(_) => -1f64,
                None => 1f64,
            };
            let num = sign * num.parse::<f64>().unwrap();
            match opt_exp {
                Some(e) => num * 10f64.powi(e as i32),
                None => num,
            }
        })
    })
}
pub fn step_udecimal(input: &str) -> Res<&str, usize> {
    context("udecimal", recognize(many1(one_of("0123456789"))))(input).map(|(next_input, res)| {
        (next_input, res.parse::<usize>().unwrap())
    })
}
pub fn step_identifier(input: &str) -> Res<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
    .map(|(next_input, res)| (next_input, res))
}
pub fn step_id(input: &str) -> Res<&str, Id> {
    context("id", preceded(tag("#"), step_udecimal))(input)
        .map(|(next_input, res)| (next_input, Id(res)))
}
pub fn step_string(input: &str) -> Res<&str, &str> {
    context(
        "string",
        delimited(tag("'"), take_till(|c| c == '\''), tag("'")),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}
pub fn step_bool(input: &str) -> Res<&str, bool> {
    context("string", delimited(tag("."), one_of("TF"), tag(".")))(input).map(
        |(next_input, res)| {
            (
                next_input,
                match res {
                    'T' => true,
                    'F' => false,
                    _ => panic!("unepected character from parser"),
                },
            )
        },
    )
}

pub fn paren_tup(input: &str) -> Res<&str, ()> {
    delimited(
        tag("("),
        separated_list0(paren_tup, take_till(|c| c == '(' || c == ')')),
        tag(")"),
    )(input)
    .map(|(next_input, _)| (next_input, ()))
}

pub fn after_ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> Res<&'a str, O>
where
    F: FnMut(&'a str) -> Res<&'a str, O>,
{
    preceded(multispace0, inner)
}
pub fn after_expect_ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> Res<&'a str, O>
where
    F: FnMut(&'a str) -> Res<&'a str, O>,
{
    preceded(multispace1, inner)
}

pub fn after_wscomma<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> Res<&'a str, O>
where
    F: FnMut(&'a str) -> Res<&'a str, O>,
{
    preceded(after_ws(tag(",")), after_ws(inner))
}

pub fn step_vec<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> Res<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> Res<&'a str, O>,
{
    delimited(
        tag("("),
        separated_list0(after_ws(tag(",")), after_ws(inner)),
        after_ws(tag(")")),
    )
}

// TODO this is wrong
pub fn step_opt<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> Res<&'a str, Option<O>>
where
    F: FnMut(&'a str) -> Res<&'a str, O>,
{
    preceded(opt(tag("$")), opt(inner))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float() {
        assert_eq!(step_float("5.xx"), Ok(("xx", 5.0)));
        assert_eq!(step_float("5.0xx"), Ok(("xx", 5.0)));
        assert_eq!(step_float("05.0.."), Ok(("..", 5.0)));
        assert_eq!(step_float("-05.0,"), Ok((",", -5.0)));
        assert_eq!(step_float("-05.0123"), Ok(("", -5.0123)));
        assert_eq!(step_float("-0.0123--"), Ok(("--", -0.0123)));
        assert!(step_float("-.0123").is_err());
        assert!(step_float(".0123").is_err());

        assert_eq!(step_float("-0.0123E-2-"), Ok(("-", -0.000123)));
        assert_eq!(step_float("-0.0123E+2e"), Ok(("e", -1.23)));
        assert_eq!(step_float("-0.0123E+002x"), Ok(("x", -1.23)));
        assert_eq!(step_float("-0.0123E+002.5"), Ok((".5", -1.23)));
        assert_eq!(step_float("0.E0"), Ok(("", 0.0)));

        assert_eq!(step_float("0.E"), Ok(("E", 0.0)));
        assert_eq!(step_float("0.E+"), Ok(("E+", 0.0)));
        assert_eq!(step_float("0.E-"), Ok(("E-", 0.0)));
        assert_eq!(step_float("0.E-E"), Ok(("E-E", 0.0)));
    }

    #[test]
    fn test_decimal() {
        assert_eq!(step_decimal("012345.."), Ok(("..", 12345)));
        assert_eq!(step_decimal("0.."), Ok(("..", 0)));
        assert_eq!(step_decimal("-0.."), Ok(("..", 0)));
        assert_eq!(step_decimal("-66-.."), Ok(("-..", -66)));
        assert!(step_decimal("-,").is_err());
        assert!(step_decimal(",").is_err());
    }
    #[test]
    fn test_udecimal() {
        assert_eq!(step_udecimal("012345.."), Ok(("..", 12345)));
        assert_eq!(step_udecimal("0.."), Ok(("..", 0)));
        assert!(step_udecimal("-0..").is_err());
        assert!(step_udecimal("-66-..").is_err());
        assert!(step_udecimal("-,").is_err());
        assert!(step_udecimal(",").is_err());
    }

    #[test]
    fn test_id() {
        assert_eq!(step_id("#012345ff"), Ok(("ff", Id(12345))));
        assert_eq!(step_id("#0"), Ok(("", Id(0))));
        assert_eq!(step_id("#10, "), Ok((", ", Id(10))));
        assert_eq!(step_id("#10#10, "), Ok(("#10, ", Id(10))));
        assert!(step_id("#,").is_err());
    }
    #[test]
    fn test_id_vec() {
        assert_eq!(
            step_vec(step_id)("(  #012345, #4556    ) ff"),
            Ok((" ff", vec![Id(12345), Id(4556)]))
        );
        assert_eq!(
            step_vec(step_id)("(  #0123 ) ff"),
            Ok((" ff", vec![Id(123)]))
        );
        assert_eq!(step_vec(step_id)("(  ) ff"), Ok((" ff", vec![])));
        assert!(step_vec(step_id)("(  #012345, #4556  ,  ) ff").is_err());
    }

    #[test]
    fn test_ws() {
        assert_eq!(
            after_ws(step_id)("  \n #012345     \n\nff"),
            Ok(("     \n\nff", Id(12345)))
        );
        assert_eq!(
            after_ws(step_id)(" \n #012345     \n\nff"),
            Ok(("     \n\nff", Id(12345)))
        );
        assert_eq!(
            after_ws(step_id)(" \t\r \n #012345     \n\nff"),
            Ok(("     \n\nff", Id(12345)))
        );
        assert_eq!(
            after_ws(step_id)(" \t\t \n #012345     \n\nff"),
            Ok(("     \n\nff", Id(12345)))
        );
        assert_eq!(after_ws(step_id)("#0"), Ok(("", Id(0))));
    }

    #[test]
    fn test_string() {
        assert_eq!(step_string("'abcd'  "), Ok(("  ", "abcd".to_string())));
        assert_eq!(step_string("'ad''dd'  "), Ok(("'dd'  ", "ad".to_string())));
        assert_eq!(step_string("'ad'dd'  "), Ok(("dd'  ", "ad".to_string())));
        assert_eq!(step_string("'',"), Ok((",", "".to_string())));
    }

    #[test]
    fn test_paren_tup() {
        assert_eq!(paren_tup("(   ( , '' ,) )abcd"), Ok(("abcd", ())));
        assert_eq!(paren_tup("(   ( ) dfasv ( , '' ,) )abcd"), Ok(("abcd", ())));
        assert_eq!(
            paren_tup("(   ( ( )(()) ) dfasv ( , '' ,) )abcd"),
            Ok(("abcd", ()))
        );
        assert_eq!(paren_tup("()abcd"), Ok(("abcd", ())));
    }
}
