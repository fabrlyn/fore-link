use nom::{branch::alt, combinator::map, multi::many0, sequence::tuple, IResult};

use crate::{
    character::{DoubleQoute, Space},
    relation_type::RelationType,
};

pub struct RelationTypes<'a>(Vec<RelationType<'a>>);

impl RelationTypes<'_> {
    pub fn parse(input: &str) -> IResult<&str, RelationTypes<'_>> {
        map(alt((many, single)), RelationTypes)(input)
    }
}

pub fn single(input: &str) -> IResult<&str, Vec<RelationType<'_>>> {
    map(RelationType::parse, |x| vec![x])(input)
}

pub fn many(input: &str) -> IResult<&str, Vec<RelationType<'_>>> {
    let (input, _) = DoubleQoute::parse(input)?;

    let (input, head) = RelationType::parse(input)?;

    let (input, tail) = many0(tuple((Space::parse, RelationType::parse)))(input)?;

    let (input, _) = DoubleQoute::parse(input)?;

    let mut all: Vec<_> = tail.into_iter().map(|x| x.1).collect();
    all.insert(0, head);

    Ok((input, all))
}
