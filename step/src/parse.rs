use std::collections::{HashSet, HashMap};
use nom::{
    branch::{alt},
    bytes::complete::{is_not, tag},
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    error::*,
    sequence::{delimited, preceded, tuple},
    multi::{separated_list0},
};
use memchr::{memchr, memchr3};
use arrayvec::ArrayVec;

use crate::{id::{Id, HasId}, ap214::{Entity, superclasses_of}};

////////////////////////////////////////////////////////////////////////////////

pub type IResult<'a, U> = nom::IResult<&'a str, U, Error<&'a str>>;

/// Helper function to generate a `nom` error result
fn nom_err<'a, U>(s: &'a str, kind: nom::error::ErrorKind) -> IResult<'a, U> {
    Err(nom::Err::Error(Error::new(s, kind)))
}

/// Helper function to generate a `nom` error result with the `Alt` tag
pub fn nom_alt_err<'a, U>(s: &'a str) -> IResult<'a, U> {
    nom_err(s, ErrorKind::Alt)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Logical(pub Option<bool>);

impl HasId for Logical {
    fn append_ids(&self, _v: &mut Vec<usize>) { /* Nothing to do here */ }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) trait Parse<'a> {
    fn parse(s: &'a str) -> IResult<'a, Self> where Self: Sized;
}

impl Parse<'_> for f64 {
    fn parse(s: &str) -> IResult<Self> {
        match fast_float::parse_partial::<f64, _>(s) {
            Err(_) => nom_err(s, ErrorKind::Float),
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
impl<'a, T: Parse<'a>, const CAP: usize> Parse<'a> for ArrayVec<T, CAP> {
    fn parse(s: &'a str) -> IResult<'a, ArrayVec<T, CAP>> {
        let (mut s, _) = char('(')(s)?;
        let mut out = ArrayVec::new();
        // Based on nom's separated_list0
        let (s_, o) = match T::parse(s) {
            Err(nom::Err::Error(_)) => return Ok((s, out)),
            e => e?,
        };
        s = s_;
        out.push(o);

        loop {
            let (s_, _) = match char(',')(s) {
                Err(nom::Err::Error(_)) => break,
                e => e?,
            };
            s = s_;
            let (s_, o) = match T::parse(s) {
                Err(nom::Err::Error(_)) => break,
                e => e?,
            };
            s = s_;
            out.push(o);
        }
        let (s, _) = char(')')(s)?;
        Ok((s, out))
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

pub(crate) trait ParseFromChunks<'a> {
    fn parse_chunks(s: &[&'a str]) -> IResult<'a, Self> where Self: Sized;
}

impl<'a, T: ParseFromChunks<'a>> Parse<'a> for T {
    fn parse(s: &'a str) -> IResult<'a, Self> {
        T::parse_chunks(&[s])
    }
}

// Simple struct so we can use param_from_chunks::<Derived> to parse a '*'
// optionally followed by a comma
pub struct Derived;
impl<'a> Parse<'a> for Derived {
    fn parse(s: &str) -> IResult<Self> {
        map(char('*'), |_| Derived)(s)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Parse a single attribute from a parameter list, consuming the trailing
/// comma (if this is midway through the list) or close parens (at the end)
///
/// The input is in the form of &str slices plus the index of the current slice,
/// for cases where we're splicing together multiple sets of arguments to build
/// a complete Entity.
fn check_str<'a>(s: &'a str, i: &mut usize, strs: &[&'a str]) -> &'a str {
    if s.is_empty() {
        *i += 1;
        strs.get(*i).unwrap_or(&"")
    } else {
        s
    }
}
pub(crate) fn param_from_chunks<'a, T: Parse<'a>>(
    last: bool, s: &'a str,
    i: &mut usize, strs: &[&'a str]) -> IResult<'a, T>
{
    let s = check_str(s, i, strs);
    let (s, out) = T::parse(s)?;
    let s = check_str(s, i, strs);
    let (s, _) = char(if last { ')'} else { ',' })(s)?;
    Ok((check_str(s, i, strs), out))
}

pub(crate) fn parse_enum_tag(s: &str) -> IResult<&str> {
    delimited(char('.'),
              nom::bytes::complete::take_while(
                  |c: char| c == '_' ||
                            c.is_ascii_uppercase() ||
                            c.is_ascii_digit()),
              char('.'))(s)
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn parse_entity_decl(s: &[u8]) -> IResult<(usize, Entity)> {
    let s = match std::str::from_utf8(s) {
        Ok(s) => s,
        Err(_) => return nom_err("", ErrorKind::Escaped), // TODO correct code?
    };
    map(tuple((Id::<()>::parse, char('='), Entity::parse)),
        |(i, _, e)| (i.0, e))(s)
}

pub(crate) fn parse_entity_fallback(s: &[u8]) -> IResult<(usize, Entity)> {
    let s = match std::str::from_utf8(s) {
        Ok(s) => s,
        Err(_) => return nom_err("", ErrorKind::Escaped),
    };
    map(Id::<()>::parse, |i| (i.0, Entity::_FailedToParse))(s)
}

pub(crate) fn parse_complex_mapping(s: &str) -> IResult<Entity> {
    // We'll maintain a map from sub-entity name to its argument string, then
    // use this map to figure out the tree and construct it.
    let mut subentities: HashMap<&str, &str> = HashMap::new();

    // Map from sub-entity name to the str slice which contains the name plus
    // the open parens, used for parsing slices
    let mut name_tags: HashMap<&str, &str> = HashMap::new();
    let bstr = s.as_bytes();
    let mut depth = 0;
    let mut index = 0;
    let mut args_start = 0;
    let mut name: &str = "";
    loop {
        let next = match memchr3(b'(', b')', b'\'', &bstr[index..]) {
            Some(i) => i,
            None => return nom_err(s, ErrorKind::Alt),
        };
        match bstr[index + next] {
            b'(' => {
                if depth == 1 {
                    let name_slice = &bstr[index..(index + next)];
                    name = std::str::from_utf8(name_slice)
                        .expect("Could not convert back to name");
                    args_start = index + next + 1;
                    let name_tag_slice = &bstr[index..(index + next + 1)];
                    let name_tag = std::str::from_utf8(name_tag_slice)
                        .expect("Could not convert tag back to name");
                    name_tags.insert(name, name_tag);
                }
                depth += 1;
            },
            b')' => {
                depth -= 1;
                if depth == 1 {
                    let arg_slice = &bstr[args_start..(index + next)];
                    let args = std::str::from_utf8(arg_slice)
                        .expect("Could not convert args");
                    subentities.insert(name, args);
                } else if depth == 0 {
                    break;
                }
            },
            b'\'' => {
                // TODO: handle escaped quotes
                let j = match memchr(b'\'', &bstr[(index + next + 1)..]) {
                    Some(j) => j,
                    None => return nom_err(s, ErrorKind::Char),
                };
                index += j + 1;
            }
            c => panic!("Invalid char: {}", c),
        }
        index += next + 1;
    }
    // Filter out the list of subclasses to those which aren't a parent of
    // another item in the set; these are our potential leafs.
    let mut potential_leafs: HashSet<&str> = subentities.keys()
        .map(|i| *i)
        .collect();
    for k in subentities.keys() {
        for sup in superclasses_of(k) {
            potential_leafs.remove(sup);
        }
    }
    // Eliminate any leaf with no arguments, since they're just addding
    // bonus constraints (which we don't handle anyways)
    potential_leafs.retain(|k| subentities[k] != "");

    // Sort potential leafs so that ComplexEntity is deterministic and we can
    // match against it later
    let mut potential_leafs: Vec<&str> = potential_leafs.into_iter().collect();
    potential_leafs.sort();

    // At this point, we'll build up argument strings by splicing together bits
    // of arguments from the existing string (to make lifetimes happy), then
    // parse into leaf entities.
    let mut leaf_entities = Vec::with_capacity(potential_leafs.len());
    for leaf in potential_leafs.into_iter() {
        let mut chain = vec![leaf];
        loop {
            let sup = superclasses_of(chain.last().unwrap());
            match sup.len() {
                0 => break,
                1 => chain.push(sup[0]),
                _ => return nom_err(s, ErrorKind::LengthValue), // TODO: error
            }
        }
        let mut new_decl: Vec<&str> = vec![name_tags.get(leaf).unwrap()];
        for c in chain.iter().rev() {
            if !subentities[c].is_empty() {
                new_decl.push(subentities[c]);
                new_decl.push(if *c == leaf { &")" } else { &"," });
            }
        }
        leaf_entities.push(Entity::parse_chunks(&new_decl)?.1)
    }
    // At this point, we assume that there's nothing left to parse, so we
    // return an empty string for the 'remaining' text
    if leaf_entities.len() == 1 {
        Ok(("", leaf_entities.pop().unwrap()))
    } else {
        Ok(("", Entity::ComplexEntity(leaf_entities)))
    }
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
        parse_entity_decl(b"#1632=(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.));").unwrap();
    }
}
