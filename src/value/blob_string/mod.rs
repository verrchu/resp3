#[cfg(test)]
mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobString(pub Bytes);

impl From<BlobString> for Value {
    fn from(input: BlobString) -> Value {
        Value::BlobString(input)
    }
}

impl BlobString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("$"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::BlobString (str::from_utf8)")
                    .and_then(|v| u64::from_str(v).context("Value::BlobString (u64::from_str)"))
            })
        };

        let parse_val = |len| terminated(take(len), tag(DELIMITER));

        parse_len
            .flat_map(parse_val)
            .map(|bytes: &[u8]| BlobString::from(bytes.to_vec()))
            .parse(input)
    }
}

impl<B: Into<Bytes>> From<B> for BlobString {
    fn from(input: B) -> Self {
        Self(input.into())
    }
}

impl TryFrom<BlobString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: BlobString) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write("$".as_bytes())
            .and_then(|_| buf.write(input.0.len().to_string().as_bytes()))
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .and_then(|_| buf.write(&input.0))
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::BlobString (buf::write)")?;
        buf.flush().context("Value::BlobString (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
