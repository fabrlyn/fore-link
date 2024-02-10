use nom::{combinator::map, IResult};

use crate::uri::Uri;

pub struct ExternalRelationType<'a>(Uri<'a>);

impl ExternalRelationType<'_> {
    pub fn parse(input: &str) -> IResult<&str, ExternalRelationType<'_>> {
        map(Uri::parse, ExternalRelationType)(input)
    }
}
