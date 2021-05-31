use nom::{
    bytes::complete::{is_not},
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    error::*,
    sequence::{delimited, preceded, tuple},
};

pub type IResult<'a, U> = nom::IResult<&'a str, U, nom::error::VerboseError<&'a str>>;

pub struct Id<T>(pub usize, std::marker::PhantomData<*const T>);
impl<T> Id<T> {
    pub fn parse(s: &str) -> IResult<Self> {
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

pub trait ParseInner<'a> {
    type Out;
    fn parse_inner(s: &'a str) -> IResult<'a, Self::Out>;
}

impl ParseInner<'_> for f64 {
    type Out = Self;
    fn parse_inner(s: &str) -> IResult<Self> {
        match fast_float::parse_partial::<f64, _>(s) {
            Err(_) => build_err(s, "Could not parse float"),
            Ok((x, n)) => Ok((&s[n..], x)),
        }
    }
}

impl ParseInner<'_> for i64 {
    type Out = Self;
    fn parse_inner(s: &str) -> IResult<Self> {
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
impl<'a> ParseInner<'a> for &'a str {
    type Out = &'a str;
    fn parse_inner(s: &'a str) -> IResult<'a, &'a str> {
        delimited(char('"'), is_not("'"), char('"'))(s)
    }
}
