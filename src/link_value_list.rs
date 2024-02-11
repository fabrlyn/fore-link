use nom::{bytes::complete::tag, multi::many0, sequence::tuple, IResult};

use crate::{link_parameter::LinkParameter, uri::Uri};

pub struct LinkValueList<'a> {
    uri: Uri<'a>,
    link_parameters: Vec<LinkParameter<'a>>,
}

impl LinkValueList<'_> {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, LinkValueList<'a>> {
        let (rest, (_, uri, _)) = tuple((tag("<"), Uri::parse, tag(">")))(input)?;
        let (rest, link_parameters) = many0(tuple((tag(";"), LinkParameter::parse)))(rest)?;

        Ok((
            rest,
            LinkValueList {
                uri,
                link_parameters: link_parameters.into_iter().map(|x| x.1).collect(),
            },
        ))
    }
}
