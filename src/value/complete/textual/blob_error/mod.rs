#[cfg(test)]
pub(crate) mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

use super::{Attribute, Value, DELIMITER};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    combinator::opt,
    sequence::{delimited, terminated},
    IResult, Parser,
};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]+").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobError {
    pub code: String,
    pub msg: Bytes,
    pub attr: Option<Attribute>,
}

impl From<BlobError> for Value {
    fn from(input: BlobError) -> Value {
        Value::BlobError(input)
    }
}

impl BlobError {
    pub fn new(code: impl Into<String>, msg: impl Into<Bytes>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
            attr: None,
        }
    }

    pub fn with_attr(mut self, attr: Attribute) -> Self {
        self.attr = Some(attr);
        self
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn msg(&self) -> &Bytes {
        &self.msg
    }

    pub fn attr(&self) -> Option<&Attribute> {
        self.attr.as_ref()
    }
}

impl BlobError {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let mut parse_len = {
            let parser = delimited(tag("!"), digit1, tag(DELIMITER));

            map_res(parser, |v: &[u8]| {
                str::from_utf8(v)
                    .context("Value::BlobError (str::from_utf8)")
                    .and_then(|v| u64::from_str(v).context("Value::BlobError (u64::from_str)"))
            })
        };

        let (input, attr) = opt(Attribute::parse).parse(input)?;
        let (input, len) = parse_len.parse(input)?;
        let (input, msg) = terminated(take(len), tag(DELIMITER)).parse(input)?;

        let code = Lazy::force(&CODE).to_owned();
        let (msg, code) = map_res(terminated(re_find(code), tag(" ")), |code: &[u8]| {
            str::from_utf8(code).context("Value::BlobError (str::from_utf8)")
        })
        .parse(msg)?;

        let mut value = BlobError::new(code, msg.to_vec());
        if let Some(attr) = attr {
            value = value.with_attr(attr);
        }

        Ok((input, value))
    }
}

impl TryFrom<&BlobError> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &BlobError) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::BlobError (Bytes::from)")?;
            buf.write(&bytes).context("Value::BlobError (buf::write)")?;
        }

        buf.write("!".as_bytes())
            .and_then(|_| {
                let len = input.code.as_bytes().len() + input.msg.len() + 1;
                buf.write(len.to_string().as_bytes())
            })
            .and_then(|_| buf.write(DELIMITER))
            .and_then(|_| buf.write(input.code.to_string().as_bytes()))
            .and_then(|_| buf.write(b" "))
            .and_then(|_| buf.write(&input.msg))
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::BlobError (buf::write)")?;

        buf.flush().context("Value::BlobError (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<BlobError> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: BlobError) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
