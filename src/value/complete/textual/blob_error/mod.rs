#[cfg(test)]
pub(crate) mod tests;

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
use nom_regex::bytes::re_find;
use once_cell::sync::Lazy;
use regex::bytes::Regex;

static CODE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]+").unwrap());

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobError {
    pub code: String,
    pub msg: Bytes,
}

impl From<BlobError> for Value {
    fn from(input: BlobError) -> Value {
        Value::BlobError(input)
    }
}

impl BlobError {
    pub(crate) fn new(code: impl Into<String>, msg: impl Into<Bytes>) -> Self {
        Self {
            code: code.into(),
            msg: msg.into(),
        }
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

        let (input, len) = parse_len.parse(input)?;
        let (input, msg) = terminated(take(len), tag(DELIMITER)).parse(input)?;

        let code = Lazy::force(&CODE).to_owned();
        let (msg, code) = map_res(terminated(re_find(code), tag(" ")), |code: &[u8]| {
            str::from_utf8(code).context("Value::BlobError (str::from_utf8)")
        })
        .parse(msg)?;

        Ok((input, BlobError::new(code, msg.to_vec())))
    }
}

impl TryFrom<BlobError> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: BlobError) -> anyhow::Result<Bytes> {
        let mut buf = vec![];
        buf.write("!".as_bytes())
            .and_then(|_| {
                let len = input.code.as_bytes().len() + input.msg.len() + 1;
                buf.write(len.to_string().as_bytes())
            })
            .and_then(|_| buf.write(b"\r\n"))
            .and_then(|_| buf.write(input.code.to_string().as_bytes()))
            .and_then(|_| buf.write(b" "))
            .and_then(|_| buf.write(&input.msg))
            .and_then(|_| buf.write(b"\r\n"))
            .context("Value::BlobError (buf::write)")?;
        buf.flush().context("Value::BlobError (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
