use nom::{bytes::complete::tag, combinator::map, IResult, Parser};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Dash<'a>(&'a str);

impl Dash<'_> {
    pub fn parse(input: &str) -> IResult<&str, Dash<'_>> {
        map(tag("-"), Dash)(input)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Dot<'a>(&'a str);

impl Dot<'_> {
    pub fn parse(input: &str) -> IResult<&str, Dot<'_>> {
        map(tag("."), Dot)(input)
    }
}
