#[cfg(test)]
mod tests;

use std::{
    io::Write,
    str::{self, FromStr},
};

use anyhow::Context;
use bytes::Bytes;
use derivative::Derivative;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    sequence::{delimited, pair, preceded, tuple},
    IResult, Parser,
};

use super::{Value, DELIMITER};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sign {
    Plus,
    Minus,
}

impl Sign {
    pub fn is_minus(&self) -> bool {
        *self == Self::Minus
    }

    pub fn is_plus(&self) -> bool {
        *self == Self::Plus
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Parts {
    sign: Sign,
    int: u64,
    frac: Option<u64>,
}

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Val {
    parts: Parts,
    #[derivative(
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Hash = "ignore"
    )]
    inner: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Double {
    Inf(Sign),
    Val(Val),
}

impl From<Double> for Value {
    fn from(input: Double) -> Value {
        Value::Double(input)
    }
}

impl From<Val> for Double {
    fn from(input: Val) -> Self {
        Self::Val(input)
    }
}

impl Double {
    fn from_parts(parts: Parts) -> anyhow::Result<Self> {
        let inner = Bytes::try_from(parts.clone())
            .context("(Value::Double) Bytes::try_from")
            .and_then(|v| {
                str::from_utf8(&v)
                    .context("Value::Double (str::from_utf8)")
                    .and_then(|v| f64::from_str(v).context("Value::Double (f64::from_str)"))
            })?;

        Ok(Self::Val(Val { parts, inner }))
    }
}

impl Double {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let parse_inf = pair(opt(tag("-")), tag("inf")).map(|(sign, _inf)| {
            sign.map(|_| Double::Inf(Sign::Minus))
                .unwrap_or(Double::Inf(Sign::Plus))
        });

        let parse_num = {
            let parser = tuple((opt(tag("-")), digit1, opt(preceded(tag("."), digit1))));

            map_res(parser, |(sign, int, frac)| {
                let sign = sign.map(|_| Sign::Minus).unwrap_or(Sign::Plus);
                let int = str::from_utf8(int)
                    .context("Value::Double (str::from_utf8)")
                    .and_then(|int| u64::from_str(int).context("Value::Double (u64::from_str)"))?;
                let frac = frac
                    .map(|frac| {
                        String::from_utf8(frac.to_vec())
                            .context("Value::Double (String::from_utf8)")
                            .and_then(|frac| {
                                u64::from_str(&frac).context("Value::Double (u64::from_str)")
                            })
                    })
                    .transpose()?;

                let parts = Parts { sign, int, frac };
                let value =
                    Double::from_parts(parts).context("Value::Double (Double::from_parts)")?;

                Ok::<_, anyhow::Error>(value)
            })
        };

        delimited(tag(","), alt((parse_num, parse_inf)), tag(DELIMITER))
            .map(Double::from)
            .parse(input)
    }
}

impl TryFrom<Double> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Double) -> anyhow::Result<Bytes> {
        let mut buf = vec![];

        buf.write(b",").context("Value::Double (buf::write)")?;

        match input {
            Double::Inf(sign) => {
                sign.is_minus()
                    .then(|| buf.write(b"-"))
                    .transpose()
                    .and_then(|_| buf.write(b"inf"))
                    .context("Value::Double (buf::write)")?;
            }
            Double::Val(val) => {
                let bytes =
                    Bytes::try_from(val.parts).context("Value::Double (Bytes::Try_from)")?;
                buf.write(&bytes).context("Value::Double (buf::write)")?;
            }
        }

        buf.write("\r\n".as_bytes())
            .context("Value::Double (buf::write)")?;

        buf.flush().context("Value::Double (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}

impl TryFrom<Parts> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Parts) -> anyhow::Result<Self> {
        let mut buf = vec![];
        input
            .sign
            .is_minus()
            .then(|| buf.write(b"-"))
            .transpose()
            .and_then(|_| buf.write(input.int.to_string().as_bytes()))
            .and_then(|_| {
                input
                    .frac
                    .map(|frac| {
                        buf.write(b".")
                            .and_then(|_| buf.write(frac.to_string().as_bytes()))
                    })
                    .transpose()
            })
            .context("Value::Double (buf::write)")?;
        buf.flush().context("Value::Double (buf::flush)")?;
        Ok(Bytes::from(buf))
    }
}
