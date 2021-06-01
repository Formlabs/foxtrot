use nom::{
    branch::{alt},
    bytes::complete::{tag, is_not},
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    error::*,
    sequence::{delimited, preceded, terminated, tuple},
    multi::{many0},
};

pub type IResult<'a, U> = nom::IResult<&'a str, U, nom::error::VerboseError<&'a str>>;

pub struct Id<T>(pub usize, std::marker::PhantomData<*const T>);
impl<'a, T> Parse<'a> for Id<T> {
    fn parse(s: &str) -> IResult<Self> {
        map_res(
            preceded(char('#'), digit1),
            |s: &str| s.parse().map(|i| Id(i, std::marker::PhantomData)))
            (s)
    }
}

fn build_err<'a, U>(s: &'a str, msg: &'static str) -> IResult<'a, U> {
    Err(nom::Err::Error(
        VerboseError {
            errors: vec![(s, VerboseErrorKind::Context(msg))]
        }))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Logical(pub Option<bool>);

////////////////////////////////////////////////////////////////////////////////

pub trait Parse<'a> {
    fn parse(s: &'a str) -> IResult<'a, Self> where Self: Sized;
}

impl Parse<'_> for f64 {
    fn parse(s: &str) -> IResult<Self> {
        match fast_float::parse_partial::<f64, _>(s) {
            Err(_) => build_err(s, "Could not parse float"),
            Ok((x, n)) => Ok((&s[n..], x)),
        }
    }
}

impl Parse<'_> for i64 {
    fn parse(s: &str) -> IResult<Self> {
        map_res(tuple((opt(char('-')), digit1)),
            |(sign, digits)| -> Result<i64, <i64 as std::str::FromStr>::Err> {
                let num = str::parse::<i64>(digits)?;
                if sign.is_some() {
                    Ok(-num)
                } else {
                    Ok(num)
                }
            })(s)
    }
}
impl<'a> Parse<'a> for &'a str {
    fn parse(s: &'a str) -> IResult<'a, &'a str> {
        delimited(char('"'), is_not("'"), char('"'))(s)
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    fn parse(s: &'a str) -> IResult<'a, Vec<T>> {
        // TODO: probably need delimiters and separators
        many0(T::parse)(s)
    }
}
impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    fn parse(s: &'a str) -> IResult<'a, Self> {
        alt((
            map(char('$'), |_| None),
            map(T::parse, |v| Some(v))))(s)
    }
}
impl<'a> Parse<'a> for Logical {
    fn parse(s: &'a str) -> IResult<'a, Self> {
        alt((
            map(tag(".TRUE."), |_| Logical(Some(true))),
            map(tag(".FALSE."), |_| Logical(Some(false))),
            map(tag(".UNKNOWN."), |_| Logical(None)),
        ))(s)
    }
}
impl<'a> Parse<'a> for bool {
    fn parse(s: &'a str) -> IResult<'a, Self> {
        alt((
            map(tag(".TRUE."), |_| true),
            map(tag(".FALSE."), |_| false),
        ))(s)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn param<'a, T: Parse<'a>>(last: bool, s: &'a str) -> IResult<'a, T> {
    terminated(T::parse, char(if last { ')'} else { ',' }))(s)
}
