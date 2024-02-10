use nom::{branch::alt, combinator::map, IResult};

use crate::{
    external_relation_type::ExternalRelationType, regular_relation_type::RegularRelationType,
};

pub enum RelationType<'a> {
    Regular(RegularRelationType<'a>),
    External(ExternalRelationType<'a>),
}

impl RelationType<'_> {
    pub fn parse(input: &str) -> IResult<&str, RelationType<'_>> {
        alt((regular, external))(input)
    }
}

fn regular(input: &str) -> IResult<&str, RelationType<'_>> {
    map(RegularRelationType::parse, RelationType::Regular)(input)
}

fn external(input: &str) -> IResult<&str, RelationType<'_>> {
    map(ExternalRelationType::parse, RelationType::External)(input)
}
