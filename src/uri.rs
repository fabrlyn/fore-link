use nom::{bytes::complete::take, combinator::map, IResult};

pub struct Uri<'a>(&'a str);

impl Uri<'_> {
    pub fn parse(input: &str) -> IResult<&str, Uri<'_>> {
        let uri = url::Url::parse(input).unwrap();
        let len = uri.as_str().len();

        map(take(len), Uri)(input)
    }
}
