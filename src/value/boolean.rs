use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};

use super::DELIMITER;

#[derive(Debug, PartialEq, Eq)]
pub struct Boolean(bool);

impl Boolean {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        delimited(tag("#"), alt((tag("t"), tag("f"))), tag(DELIMITER))
            .map(|value| match value {
                "f" => Boolean(false),
                "t" => Boolean(true),
                _ => unreachable!(),
            })
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Boolean::parse("#t\r\n"), Ok(("", Boolean(true))));
        assert_eq!(Boolean::parse("#f\r\n"), Ok(("", Boolean(false))));
    }
}
