#[cfg(test)]
pub(crate) mod tests;

use std::io::Write;

use anyhow::Context;
use bytes::Bytes;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::opt,
    sequence::{delimited, pair},
    IResult, Parser,
};

use super::{Attribute, Value, DELIMITER};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Boolean {
    val: bool,
    attr: Option<Attribute>,
}

impl From<Boolean> for Value {
    fn from(input: Boolean) -> Value {
        Value::Boolean(input)
    }
}

impl Boolean {
    pub fn val(&self) -> bool {
        self.val
    }

    pub fn attr(&self) -> Option<&Attribute> {
        self.attr.as_ref()
    }

    pub fn with_attr(mut self, attr: Attribute) -> Self {
        self.attr = Some(attr);
        self
    }
}

impl Boolean {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_attr = opt(Attribute::parse);
        let parse_val = delimited(tag("#"), alt((tag("t"), tag("f"))), tag(DELIMITER));

        pair(parse_attr, parse_val)
            .map(|(attr, val)| {
                let mut value = match val {
                    b"f" => Boolean::from(false),
                    b"t" => Boolean::from(true),
                    _ => unreachable!(),
                };

                value.attr = attr;

                value
            })
            .parse(input)
    }
}

impl From<bool> for Boolean {
    fn from(input: bool) -> Self {
        Self {
            val: input,
            attr: None,
        }
    }
}

impl TryFrom<&Boolean> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Boolean) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        if let Some(attr) = input.attr.as_ref() {
            let bytes = Bytes::try_from(attr).context("Value::Boolean (Bytes::from)")?;
            buf.write(&bytes).context("Value::Boolean (buf::write)")?;
        }

        buf.write(b"#")
            .and_then(|_| match input.val() {
                true => buf.write(b"t"),
                false => buf.write(b"f"),
            })
            .and_then(|_| buf.write(DELIMITER))
            .context("Value::Boolean (buf::write)")?;
        buf.flush().context("Value::Boolean (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<Boolean> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Boolean) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
