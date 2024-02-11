use nom::{combinator::map, multi::many1, IResult};

use crate::ptoken_char::PTokenChar;

pub struct PToken(Vec<PTokenChar>);

impl PToken {
    pub fn parse(input: &str) -> IResult<&str, PToken> {
        map(many1(PTokenChar::parse), PToken)(input)
    }
}

pub fn parse(input: &str) -> IResult<&str, &str> {
    todo!()
}
