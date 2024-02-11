use nom::{bytes::complete::tag, sequence::tuple, IResult};

use crate::{parameter_name::ParameterName, parameter_value::ParameterValue};

pub struct LinkExtension<'a> {
    pub name: ParameterName<'a>,
    pub value: ParameterValue<'a>,
}

impl LinkExtension<'_> {
    pub fn parse(input: &str) -> IResult<&str, LinkExtension<'_>> {
        let (rest, (name, _, value)) =
            tuple((ParameterName::parse, tag("="), ParameterValue::parse))(input)?;

        Ok((rest, LinkExtension { name, value }))
    }
}
