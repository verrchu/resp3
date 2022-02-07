#[cfg(test)]
mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerbatimString {
    Txt(Bytes),
    Mkd(Bytes),
}

impl From<VerbatimString> for Value {
    fn from(input: VerbatimString) -> Value {
        Value::VerbatimString(input)
    }
}

impl VerbatimString {
    pub fn txt(bytes: impl Into<Bytes>) -> Self {
        Self::Txt(bytes.into())
    }

    pub fn mkd(bytes: impl Into<Bytes>) -> Self {
        Self::Mkd(bytes.into())
    }

    fn tag(&self) -> &'static str {
        match self {
            Self::Txt(_) => "txt",
            Self::Mkd(_) => "mkd",
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Txt(bytes) => bytes.len(),
            Self::Mkd(bytes) => bytes.len(),
        }
    }

    fn bytes(&self) -> &[u8] {
        match self {
            Self::Txt(bytes) => bytes,
            Self::Mkd(bytes) => bytes,
        }
    }
}

impl VerbatimString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_len = {
            let parser = delimited(tag("="), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::Array (str::from_utf8)")
                    .and_then(|v| u64::from_str(v).context("Value::Array (u64::from_str)"))
            })
        };

        let parse_msg = |len: u64| {
            terminated(
                separated_pair(alt((tag("txt"), tag("mkd"))), tag(":"), take(len - 4)),
                tag(DELIMITER),
            )
        };

        parse_len
            .flat_map(parse_msg)
            .map(|(ty, msg)| match ty {
                b"txt" => VerbatimString::Txt(Bytes::from(msg.to_vec())),
                b"mkd" => VerbatimString::Mkd(Bytes::from(msg.to_vec())),
                _ => unreachable!(),
            })
            .parse(input)
    }
}

impl TryFrom<VerbatimString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: VerbatimString) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write("=".as_bytes())
            .and_then(|_| buf.write((input.len() + 4).to_string().as_bytes()))
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .and_then(|_| buf.write(input.tag().as_bytes()))
            .and_then(|_| buf.write(":".as_bytes()))
            .and_then(|_| buf.write(input.bytes()))
            .and_then(|_| buf.write("\r\n".as_bytes()))
            .context("Value::VerbatimString (buf::write)")?;
        buf.flush().context("Value::VerbatimString (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
