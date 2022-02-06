use nom::{bytes::complete::tag, sequence::terminated, IResult, Parser};

use super::DELIMITER;

#[derive(Debug, PartialEq, Eq)]
pub struct Null;

impl Null {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        terminated(tag("_"), tag(DELIMITER))
            .map(|_| Null)
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Null::parse("_\r\n"), Ok(("", Null)));
    }
}
