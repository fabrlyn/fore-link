use nom::{combinator::map, IResult};

use crate::link_extension::LinkExtension;

pub enum LinkParameter<'a> {
    LinkExtension(LinkExtension<'a>),
}

impl LinkParameter<'_> {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, LinkParameter<'a>> {
        //alt((...))
        link_extension(input)
    }
}

fn link_extension(input: &str) -> IResult<&str, LinkParameter<'_>> {
    map(LinkExtension::parse, LinkParameter::LinkExtension)(input)
}
