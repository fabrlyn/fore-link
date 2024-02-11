use nom::IResult;

pub struct QoutedString<'a>(&'a str);

impl QoutedString<'_> {
    pub fn parse(input: &str) -> IResult<&str, QoutedString<'_>> {
        todo!()
    }
}

pub fn parse(input: &str) -> IResult<&str, &str> {
    todo!()
}
