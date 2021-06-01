use memchr::{memchr, memchr2, memchr_iter};
use nom::{
    branch::{alt},
    bytes::complete::{tag, is_not},
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    error::*,
    sequence::{delimited, preceded, terminated, tuple},
    multi::{separated_list0},
};
use crate::ap214::Entity;

pub type IResult<'a, U> = nom::IResult<&'a str, U, nom::error::VerboseError<&'a str>>;

#[derive(Debug)]
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

/// Parse a single attribute from a parameter list, consuming the trailing
/// comma (if this is midway through the list) or close parens (at the end)
pub fn param<'a, T: Parse<'a>>(last: bool, s: &'a str) -> IResult<'a, T> {
    terminated(T::parse, char(if last { ')'} else { ',' }))(s)
}

////////////////////////////////////////////////////////////////////////////////

/// Flattens a STEP file, removing comments and whitespace
pub fn strip_flatten(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        match data[i] {
            b'/' => if i + 1 < data.len() && data[i + 1] == b'*' {
                for j in memchr_iter(b'/', &data[i + 2..]) {
                    if data[i + j + 1] == b'*' {
                        i += j + 2;
                        break;
                    }
                }
            }
            c if c.is_ascii_whitespace() => (),
            c => out.push(c),
        }
        i += 1;
    }
    out
}

pub fn into_blocks(data: &[u8]) -> Vec<&[u8]> {
    let mut blocks = Vec::new();
    let mut i = 0;
    let mut start = 0;
    while i < data.len() {
        let next = memchr2(b'\'', b';', &data[i..]).unwrap();
        match data[i + next] {
            // Skip over quoted blocks
            b'\'' => i += next + memchr(b'\'', &data[i + next..]).unwrap() + 1,
            b';' => {
                blocks.push(&data[start..=(i + next)]);

                i += next + 1; // Skip the semicolon
                start = i;
            },
            _ => unreachable!(),
        }
    }
    blocks
}

pub fn parse_entity_decl(s: &[u8]) -> IResult<(usize, Entity)> {
    let s = match std::str::from_utf8(s) {
        Ok(s) => s,
        Err(_) => return build_err("", "Invalid unicode"),
    };
    map(tuple((Id::<()>::parse, char('='), Entity::parse)),
        |(i, _, e)| (i.0, e))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_entity_decl() {
        println!("{:?}", parse_entity_decl(b"#3=SHAPE_DEFINITION_REPRESENTATION(#4,#10);"));
        assert!(parse_entity_decl(b"#3=SHAPE_DEFINITION_REPRESENTATION(#4,#10);").is_ok());
    }
}
