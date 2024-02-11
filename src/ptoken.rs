use nom::IResult;

pub struct PToken<'a>(&'a str);

impl<'a> PToken<'a> {
    pub fn parse(input: &str) -> IResult<&str, PToken<'_>> {
        todo!()
    }
}

pub fn parse(input: &str) -> IResult<&str, &str> {
    todo!()
}
