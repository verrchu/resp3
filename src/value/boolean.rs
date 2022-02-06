use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};

use super::{Value, DELIMITER};

#[derive(Debug, PartialEq, Eq)]
pub struct Boolean(pub bool);

impl From<Boolean> for Value {
    fn from(input: Boolean) -> Value {
        Value::Boolean(input)
    }
}

impl Boolean {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        delimited(tag("#"), alt((tag("t"), tag("f"))), tag(DELIMITER))
            .map(|value: &[u8]| match value {
                b"f" => Boolean(false),
                b"t" => Boolean(true),
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
        assert_eq!(
            Boolean::parse(&b"#t\r\n"[..]),
            Ok((&b""[..], Boolean(true)))
        );

        assert_eq!(
            Boolean::parse(&b"#f\r\n"[..]),
            Ok((&b""[..], Boolean(false)))
        );
    }
}
