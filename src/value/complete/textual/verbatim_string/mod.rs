#[cfg(test)]
pub(crate) mod tests;

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
    combinator::{map_res, opt},
    sequence::{delimited, pair, separated_pair, terminated},
    IResult, Parser,
};

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerbatimString {
    Txt { val: Bytes, attr: Option<Attribute> },
    Mkd { val: Bytes, attr: Option<Attribute> },
}

impl From<VerbatimString> for Value {
    fn from(input: VerbatimString) -> Value {
        Value::VerbatimString(input)
    }
}

impl VerbatimString {
    pub fn txt(bytes: impl Into<Bytes>) -> Self {
        Self::Txt {
            val: bytes.into(),
            attr: None,
        }
    }

    pub fn mkd(bytes: impl Into<Bytes>) -> Self {
        Self::Mkd {
            val: bytes.into(),
            attr: None,
        }
    }

    pub fn with_attr(self, attr: Attribute) -> Self {
        match self {
            Self::Mkd { val, .. } => Self::Mkd {
                val,
                attr: Some(attr),
            },
            Self::Txt { val, .. } => Self::Txt {
                val,
                attr: Some(attr),
            },
        }
    }

    fn tag(&self) -> &'static str {
        match self {
            Self::Txt { .. } => "txt",
            Self::Mkd { .. } => "mkd",
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Txt { val, .. } => val.len(),
            Self::Mkd { val, .. } => val.len(),
        }
    }

    pub fn val(&self) -> &Bytes {
        match self {
            Self::Txt { val, .. } => val,
            Self::Mkd { val, .. } => val,
        }
    }

    pub fn attr(&self) -> Option<&Attribute> {
        match self {
            Self::Txt { attr, .. } => attr.as_ref(),
            Self::Mkd { attr, .. } => attr.as_ref(),
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

        let parse_attr = opt(Attribute::parse);
        let parse_val = parse_len.flat_map(parse_msg);

        pair(parse_attr, parse_val)
            .map(|(attr, (ty, msg))| {
                let mut value = match ty {
                    b"txt" => VerbatimString::txt(msg.to_vec()),
                    b"mkd" => VerbatimString::mkd(msg.to_vec()),
                    _ => unreachable!(),
                };

                if let Some(attr) = attr {
                    value = value.with_attr(attr);
                }

                value
            })
            .parse(input)
    }
}

impl TryFrom<&VerbatimString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &VerbatimString) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr() {
            let bytes = Bytes::try_from(attr).context("Value::VerbatimString (Bytes::from)")?;
            buf.write(&bytes)
                .context("Value::VerbatimString (buf::write)")?;
        }

        buf.write(b"=")
            .and_then(|_| buf.write((input.len() + 4).to_string().as_bytes()))
            .and_then(|_| buf.write(DELIMITER))
            .and_then(|_| buf.write(input.tag().as_bytes()))
            .and_then(|_| buf.write(b":"))
            .and_then(|_| buf.write(input.val()))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::VerbatimString (buf::write)")?;

        buf.flush().context("Value::VerbatimString (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<VerbatimString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: VerbatimString) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
