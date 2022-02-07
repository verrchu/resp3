#[cfg(test)]
mod tests;

use std::{io::Write, str};

use anyhow::Context;
use bytes::Bytes;
use nom::{bytes::complete::tag, combinator::map_res, sequence::delimited, IResult, Parser};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());

use super::{Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleString(String);

impl From<SimpleString> for Value {
    fn from(input: SimpleString) -> Value {
        Value::SimpleString(input)
    }
}

impl SimpleString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let re = Lazy::force(&RE).to_owned();

        let parser = delimited(tag("+"), re_find(re), tag(DELIMITER));
        let wrapper = map_res(parser, |v: &[u8]| {
            str::from_utf8(v).context("Value::SimpleString (str::from_utf8)")
        });

        wrapper.map(SimpleString::from).parse(input)
    }
}

impl<S: Into<String>> From<S> for SimpleString {
    fn from(input: S) -> Self {
        Self(input.into())
    }
}

impl TryFrom<SimpleString> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: SimpleString) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write("+".as_bytes())
            .context("Value::SimpleString (buf::write)")?;
        buf.write(input.0.as_bytes())
            .context("Value::SimpleString (buf::write)")?;
        buf.write("\r\n".as_bytes())
            .context("Value::SimpleString (buf::write)")?;
        buf.flush().context("Value::SimpleString (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
