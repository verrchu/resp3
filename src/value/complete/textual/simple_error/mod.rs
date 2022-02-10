#[cfg(test)]
pub(crate) mod tests;

use std::{io::Write, str};

use anyhow::Context;
use bytes::Bytes;
use nom::{
    bytes::complete::tag,
    combinator::map_res,
    combinator::opt,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult, Parser,
};
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static MSG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\r\n]+").unwrap());
static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]+").unwrap());

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleError {
    code: String,
    msg: String,
    attr: Option<Attribute>,
}

impl From<SimpleError> for Value {
    fn from(input: SimpleError) -> Value {
        Value::SimpleError(input)
    }
}

impl SimpleError {
    pub fn new(code: impl Into<String>, msg: impl Into<String>) -> Self {
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

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn attr(&self) -> Option<&Attribute> {
        self.attr.as_ref()
    }
}

impl SimpleError {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let msg = Lazy::force(&MSG).to_owned();
        let code = Lazy::force(&CODE).to_owned();

        let parse_code = preceded(tag("-"), re_find(code));
        let parse_msg = terminated(re_find(msg), tag(DELIMITER));

        let parse_attr = opt(Attribute::parse);
        let parse_val = separated_pair(parse_code, tag(" "), parse_msg);

        let mut wrapper = map_res(pair(parse_attr, parse_val), |(attr, (code, msg))| {
            let code = str::from_utf8(code).context("Value::SimpleError (str::from_utf8")?;
            let msg = str::from_utf8(msg).context("Value::SimpleError (str::from_utf8")?;

            let mut value = SimpleError::new(code, msg);
            if let Some(attr) = attr {
                value = value.with_attr(attr);
            }

            Ok::<_, anyhow::Error>(value)
        });

        wrapper.parse(input)
    }
}

impl TryFrom<&SimpleError> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &SimpleError) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::SimpleError (Bytes::from)")?;
            buf.write(&bytes)
                .context("Value::SimpleError (buf::write)")?;
        }

        buf.write(b"-")
            .and_then(|_| buf.write(input.code.as_bytes()))
            .and_then(|_| buf.write(b" "))
            .and_then(|_| buf.write(input.msg.as_bytes()))
            .and_then(|_| buf.write(b"\r\n"))
            .context("Value::SimpleError (buf::write)")?;

        buf.flush().context("Value::SimpleError (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<SimpleError> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: SimpleError) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
