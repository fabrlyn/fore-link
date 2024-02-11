use nom::{
    bytes::complete::{tag, take_till},
    sequence::tuple,
    IResult,
};

pub struct QuotedString<'a>(&'a str);

impl QuotedString<'_> {
    // TODO: Not sure about this implementation
    pub fn parse(input: &str) -> IResult<&str, QuotedString<'_>> {
        let (rest, _) = tag("\"")(input)?;
        let (rest, value) = take_till(|c| c == '"')(rest)?;
        let (rest, _) = tag("\"")(rest)?;

        Ok((rest, QuotedString(value)))
    }
}

pub fn parse(input: &str) -> IResult<&str, &str> {
    todo!()
}
