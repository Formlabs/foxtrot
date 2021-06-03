use nom::{
    branch::{alt},
    bytes::complete::{tag, is_not},
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    error::*,
    sequence::{delimited, preceded, terminated, tuple},
    multi::{separated_list0},
};

use crate::{id::Id, ap214::Entity};

////////////////////////////////////////////////////////////////////////////////

pub type IResult<'a, U> = nom::IResult<&'a str, U, VerboseError<&'a str>>;

/// Helper function to generate a `nom` error result
fn nom_err<'a, U>(s: &'a str, msg: &'static str) -> IResult<'a, U> {
    Err(nom::Err::Error(
        VerboseError {
            errors: vec![(s, VerboseErrorKind::Context(msg))]
        }))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Logical(pub Option<bool>);

////////////////////////////////////////////////////////////////////////////////

pub(crate) trait Parse<'a> {
    fn parse(s: &'a str) -> IResult<'a, Self> where Self: Sized;
}

impl Parse<'_> for f64 {
    fn parse(s: &str) -> IResult<Self> {
        match fast_float::parse_partial::<f64, _>(s) {
            Err(_) => nom_err(s, "Could not parse float"),
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
        alt((
            map(delimited(char('\''), opt(is_not("'")), char('\'')),
                |r| r.unwrap_or("")),
            // NUL REF
            map(char('$'), |_| "")))(s)
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    fn parse(s: &'a str) -> IResult<'a, Vec<T>> {
        delimited(char('('), separated_list0(char(','), T::parse), char(')'))(s)
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
            map(tag(".T."), |_| Logical(Some(true))),
            map(tag(".F."), |_| Logical(Some(false))),
            map(tag(".UNKNOWN."), |_| Logical(None)),
        ))(s)
    }
}
impl<'a> Parse<'a> for bool {
    fn parse(s: &'a str) -> IResult<'a, Self> {
        alt((
            map(tag(".T."), |_| true),
            map(tag(".F."), |_| false),
        ))(s)
    }
}
impl<'a, T> Parse<'a> for Id<T> {
    fn parse(s: &str) -> IResult<Self> {
        alt((
            map_res(
                preceded(char('#'), digit1),
                |s: &str| s.parse().map(|i| Id::new(i))),
            // NUL id deserializes to 0
            map(char('$'), |_| Id::empty())))
            (s)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Parse a single attribute from a parameter list, consuming the trailing
/// comma (if this is midway through the list) or close parens (at the end)
pub(crate) fn param<'a, T: Parse<'a>>(last: bool, s: &'a str) -> IResult<'a, T> {
    terminated(T::parse, char(if last { ')'} else { ',' }))(s)
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn parse_entity_decl(s: &[u8]) -> IResult<(usize, Entity)> {
    let s = match std::str::from_utf8(s) {
        Ok(s) => s,
        Err(_) => return nom_err("", "Invalid unicode"),
    };
    map(tuple((Id::<()>::parse, char('='), Entity::parse)),
        |(i, _, e)| (i.0, e))(s)
}

pub(crate) fn parse_entity_fallback(s: &[u8]) -> IResult<(usize, Entity)> {
    let s = match std::str::from_utf8(s) {
        Ok(s) => s,
        Err(_) => return nom_err("", "Invalid unicode"),
    };
    map(Id::<()>::parse, |i| (i.0, Entity::_FailedToParse))(s)
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_entity_decl() {
        parse_entity_decl(b"#3=SHAPE_DEFINITION_REPRESENTATION(#4,#10);").unwrap();
        parse_entity_decl(b"#38463=ADVANCED_FACE('',(#38464),#38475,.F.);").unwrap();
        parse_entity_decl(b"#395359=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#395356,'distance_accuracy_value','confusion accuracy');").unwrap();
    }
}
