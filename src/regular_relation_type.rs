use nom::{branch::alt, combinator::map, multi::many0, sequence::tuple, IResult};
use serde::Serialize;

use crate::{
    character::{Dash, Dot},
    lowercase_alpha::LowercaseAlpha,
    number::Number,
};

#[derive(Clone, Debug, Serialize)]
pub struct RegularRelationType<'a>(LowercaseAlpha, Vec<Component<'a>>);

impl RegularRelationType<'_> {
    fn parse(input: &str) -> IResult<&str, RegularRelationType<'_>> {
        map(
            tuple((LowercaseAlpha::parse, many0(Component::parse))),
            |(a, b)| RegularRelationType(a, b),
        )(input)
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum Component<'a> {
    LowercaseAlpha(LowercaseAlpha),
    Number(Number<'a>),
    Dot(Dot<'a>),
    Dash(Dash<'a>),
}

impl Component<'_> {
    fn parse(input: &str) -> IResult<&str, Component<'_>> {
        alt((
            map(LowercaseAlpha::parse, Component::LowercaseAlpha),
            map(Number::parse, Component::Number),
            map(Dot::parse, Component::Dot),
            map(Dash::parse, Component::Dash),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::RegularRelationType;

    #[test]
    fn test() {
        let (rest, result) = RegularRelationType::parse("abc.123-asdf.123-123").unwrap();
        println!("{:#?}", result);
    }
}
