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
use rayon::prelude::*;

use crate::ap214::Entity;

pub type IResult<'a, U> = nom::IResult<&'a str, U, VerboseError<&'a str>>;

#[derive(Debug)]
pub struct Id<T>(pub usize, std::marker::PhantomData<*const T>);
impl<'a, T> Parse<'a> for Id<T> {
    fn parse(s: &str) -> IResult<Self> {
        alt((
            map_res(
                preceded(char('#'), digit1),
                |s: &str| s.parse().map(|i| Id(i, std::marker::PhantomData))),
            // NUL id deserializes to 0
            map(char('$'), |_| Id(0, std::marker::PhantomData))))
            (s)
    }
}
unsafe impl<T> Sync for Id<T> {}
unsafe impl<T> Send for Id<T> {}

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

pub trait Parse<'a> {
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

////////////////////////////////////////////////////////////////////////////////

/// Parse a single attribute from a parameter list, consuming the trailing
/// comma (if this is midway through the list) or close parens (at the end)
pub fn param<'a, T: Parse<'a>>(last: bool, s: &'a str) -> IResult<'a, T> {
    terminated(T::parse, char(if last { ')'} else { ',' }))(s)
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct StepFile<'a>(pub Vec<Entity<'a>>);
impl<'a> StepFile<'a> {
    /// Parses a STEP file from a raw array of bytes
    /// `data` must be preprocessed by [`strip_flatten`] first
    pub fn parse(data: &'a [u8]) -> Self {
        let blocks = Self::into_blocks(&data);
        let data_start = blocks.iter()
            .position(|b| b == b"DATA;")
            .unwrap_or(0) + 1;
        let data_end = blocks.iter()
            .skip(data_start)
            .position(|b| b == b"ENDSEC;")
            .unwrap_or(0) + data_start;

        Self(blocks[data_start..data_end]
            .par_iter()
            .map(|b| Self::parse_entity_decl(*b).unwrap().1.1)
            .collect())
    }

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

    /// Splits a STEP file into individual blocks.  The input must be pre-processed
    /// by [`strip_flatten`] beforehand.
    fn into_blocks(data: &[u8]) -> Vec<&[u8]> {
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

    fn parse_entity_decl(s: &[u8]) -> IResult<(usize, Entity)> {
        let s = match std::str::from_utf8(s) {
            Ok(s) => s,
            Err(_) => return nom_err("", "Invalid unicode"),
        };
        map(tuple((Id::<()>::parse, char('='), Entity::parse)),
            |(i, _, e)| (i.0, e))(s)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_entity_decl() {
        StepFile::parse_entity_decl(b"#3=SHAPE_DEFINITION_REPRESENTATION(#4,#10);").unwrap();
        StepFile::parse_entity_decl(b"#38463=ADVANCED_FACE('',(#38464),#38475,.F.);").unwrap();
        StepFile::parse_entity_decl(b"#395359=UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-007),#395356,'distance_accuracy_value','confusion accuracy');").unwrap();
    }
}
