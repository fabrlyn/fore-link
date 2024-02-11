use fmedia_type::MediaType;
use nom::{combinator::map, sequence::tuple, IResult};

use crate::character::DoubleQoute;

pub struct QoutedMediaType<'a>(MediaType<'a>);

impl QoutedMediaType<'_> {
    pub fn parse(input: &str) -> IResult<&str, QoutedMediaType<'_>> {
        map(
            tuple((DoubleQoute::parse, media_type, DoubleQoute::parse)),
            |e| QoutedMediaType(e.1),
        )(input)
    }
}

fn media_type(input: &str) -> IResult<&str, MediaType<'_>> {
    MediaType::parse(input).ok_or(nom::Err::Error(nom::error::Error::new(
        "Not a media type",
        nom::error::ErrorKind::Fail,
    )))
}
