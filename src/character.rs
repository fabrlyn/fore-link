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

#[derive(Clone, Debug, Serialize)]
pub struct DoubleQoute<'a>(&'a str);

impl DoubleQoute<'_> {
    pub fn parse(input: &str) -> IResult<&str, DoubleQoute<'_>> {
        map(tag("\""), DoubleQoute)(input)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Space<'a>(&'a str);

impl Space<'_> {
    pub fn parse(input: &str) -> IResult<&str, Space<'_>> {
        map(tag(" "), Space)(input)
    }
}
