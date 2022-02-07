use std::str::{self, FromStr};

use super::{Value, DELIMITER};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, terminated},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobString(pub Bytes);

impl From<BlobString> for Value {
    fn from(input: BlobString) -> Value {
        Value::BlobString(input)
    }
}

impl BlobString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let mut parse_len = {
            let parser = delimited(tag("$"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::BlobString (str::from_utf8)")
                    .and_then(|v| u64::from_str(v).context("Value::BlobString (u64::from_str)"))
            })
        };

        let (input, len) = parse_len.parse(input)?;

        terminated(take(len), tag(DELIMITER))
            .map(|bytes: &[u8]| BlobString::from(bytes.to_vec()))
            .parse(input)
    }
}

impl<B: Into<Bytes>> From<B> for BlobString {
    fn from(input: B) -> Self {
        Self(input.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            BlobString::parse(&b"$11\r\nhello world\r\n"[..]),
            Ok((&b""[..], BlobString(Bytes::from(b"hello world".to_vec()))))
        );
    }
}
