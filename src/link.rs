use nom::{bytes::complete::tag, multi::many0, sequence::tuple, IResult};

use crate::link_value_list::LinkValueList;

pub struct Link<'a>(Vec<LinkValueList<'a>>);

impl Link<'_> {
    pub fn parse(input: &str) -> IResult<&str, Link<'_>> {
        let (rest, head) = LinkValueList::parse(input)?;
        let (rest, tail) = many0(tuple((tag(","), LinkValueList::parse)))(rest)?;

        let mut all: Vec<_> = tail.into_iter().map(|x| x.1).collect();
        all.insert(0, head);

        Ok((rest, Link(all)))
    }
}
