use nom::{character::complete::digit1, combinator::map, IResult};
use serde::Serialize;

// Sould be Digit
#[derive(Clone, Debug, Serialize)]
pub struct Number<'a>(&'a str);

impl Number<'_> {
    pub fn parse(input: &str) -> IResult<&str, Number<'_>> {
        map(digit1, Number)(input)
    }
}
