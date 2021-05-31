use nom::{
    character::complete::{char, digit1},
    combinator::{map_res},
    sequence::{preceded},
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
