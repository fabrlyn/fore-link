use nom::{branch::alt, combinator::map, IResult};

use crate::{
    ptoken::{self, PToken},
    quoted_string::{self, QuotedString},
};

pub struct ParameterValue<'a>(&'a str);

impl ParameterValue<'_> {
    pub fn parse(input: &str) -> IResult<&str, ParameterValue<'_>> {
        map(alt((ptoken::parse, quoted_string::parse)), ParameterValue)(input)
    }
}
