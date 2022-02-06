use nom::{
    bytes::complete::tag, character::complete::alpha1, sequence::delimited, IResult, Parser,
};

use super::DELIMITER;

#[derive(Debug)]
pub struct SimpleError(String);

impl SimpleError {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(tag("-"), alpha1, tag(DELIMITER))
            .map(Self::from)
            .parse(input)
    }
}

impl<'a> From<&'a str> for SimpleError {
    fn from(input: &'a str) -> Self {
        Self(input.to_string())
    }
}
