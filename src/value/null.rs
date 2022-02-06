use nom::{bytes::complete::tag, sequence::terminated, IResult, Parser};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null;

impl From<Null> for Value {
    fn from(_input: Null) -> Value {
        Value::Null
    }
}

impl Null {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
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
        assert_eq!(Null::parse(&b"_\r\n"[..]), Ok((&b""[..], Null)));
    }
}
