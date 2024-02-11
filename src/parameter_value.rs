use nom::{branch::alt, combinator::map, IResult};

use crate::{
    ptoken::{self, PToken},
    qouted_string::{self, QoutedString},
};

pub struct ParameterValue<'a>(&'a str);

impl ParameterValue<'_> {
    pub fn parse(input: &str) -> IResult<&str, ParameterValue<'_>> {
        map(alt((ptoken::parse, qouted_string::parse)), ParameterValue)(input)
    }
}
