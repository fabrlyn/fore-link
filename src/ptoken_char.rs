use nom::{character::complete::one_of, combinator::map, IResult};

pub struct PTokenChar(char);

impl PTokenChar {
    pub fn parse(input: &str) -> IResult<&str, PTokenChar> {
        map(
            one_of("!#$%&'()*+-./:<=>?@[]^_`{|}~abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"),
            PTokenChar,
        )(input)
    }
}
