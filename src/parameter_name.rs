use nom::IResult;

pub struct ParameterName<'a>(&'a str);

impl ParameterName<'_>{ 
    pub fn parse(input: &str) -> IResult<&str, ParameterName<'_>>{ 
        todo!()
    }
}
