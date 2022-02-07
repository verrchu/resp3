#[cfg(test)]
mod tests;

use std::io::Write;

use anyhow::Context;
use bytes::Bytes;
use nom::{branch::alt, bytes::complete::tag, sequence::delimited, IResult, Parser};

use super::{Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl From<bool> for Boolean {
    fn from(input: bool) -> Self {
        Self(input)
    }
}

impl TryFrom<Boolean> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Boolean) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write("#".as_bytes())
            .and_then(|_| {
                let bool_str = match input.0 {
                    true => "t",
                    false => "f",
                };

                buf.write(bool_str.as_bytes())
            })
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::Boolean (buf::write)")?;
        buf.flush().context("Value::Boolean (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
