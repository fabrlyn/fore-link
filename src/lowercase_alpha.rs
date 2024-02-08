use nom::{
    bytes::complete::take,
    character::complete::{alpha1, one_of},
    combinator::map,
    IResult,
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct LowercaseAlpha(char);

impl LowercaseAlpha {
    pub fn parse(input: &str) -> IResult<&str, LowercaseAlpha> {
        map(one_of("abcdefghijklmnopqrstuvwxyz"), LowercaseAlpha)(input)
    }
}
