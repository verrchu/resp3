#[cfg(test)]
pub(crate) mod tests;

use std::{io::Write, str};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag,
    combinator::{map_res, opt},
    sequence::{delimited, pair},
    IResult, Parser,
};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleString {
    val: String,
    attr: Option<Attribute>,
}

impl From<SimpleString> for Value {
    fn from(input: SimpleString) -> Value {
        Value::SimpleString(input)
    }
}

impl SimpleString {
    pub fn with_attr(mut self, attr: Attribute) -> Self {
        self.attr = Some(attr);
        self
    }

    pub fn val(&self) -> &str {
        &self.val
    }

    pub fn attr(&self) -> Option<&Attribute> {
        self.attr.as_ref()
    }
}

impl SimpleString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let re = Lazy::force(&RE).to_owned();

        let parse_val = delimited(tag("+"), re_find(re), tag(DELIMITER));
        let parse_val = map_res(parse_val, |v: &[u8]| {
            str::from_utf8(v)
                .map(ToString::to_string)
                .context("Value::SimpleString (str::from_utf8)")
        });

        let parse_attr = opt(Attribute::parse);
        pair(parse_attr, parse_val)
            .map(|(attr, val)| SimpleString { val, attr })
            .parse(input)
    }
}

impl<S: Into<String>> From<S> for SimpleString {
    fn from(val: S) -> Self {
        Self {
            val: val.into(),
            attr: None,
        }
    }
}

impl TryFrom<&SimpleString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &SimpleString) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::SimpleString (Bytes::from)")?;
            buf.write(&bytes)
                .context("Value::SimpleString (buf::write)")?;
        }

        buf.write(b"+")
            .and_then(|_| buf.write(input.val().as_bytes()))
            .and_then(|_| buf.write(b"\r\n"))
            .context("Value::SimpleString (buf::write)")?;

        buf.flush().context("Value::SimpleString (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<SimpleString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: SimpleString) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
