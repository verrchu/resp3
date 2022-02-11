#[cfg(test)]
pub(crate) mod tests;

mod complete;

pub use complete::{
    primitive::{BigNumber, Boolean, Double, Null, Number},
    recursive::{Array, Map, Set},
    special::Attribute,
    textual::{BlobError, BlobString, SimpleError, SimpleString, VerbatimString},
};

use bytes::Bytes;
use nom::{branch::alt, IResult, Parser};

static DELIMITER: &[u8] = b"\r\n";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Array(Array),
    BigNumber(BigNumber),
    BlobError(BlobError),
    BlobString(BlobString),
    Boolean(Boolean),
    Double(Double),
    Map(Map),
    Null(Null),
    Number(Number),
    Set(Set),
    SimpleError(SimpleError),
    SimpleString(SimpleString),
    VerbatimString(VerbatimString),
}

impl Value {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        alt((
            Array::parse.map(Value::from),
            BigNumber::parse.map(Value::from),
            BlobError::parse.map(Value::from),
            BlobString::parse.map(Value::from),
            Boolean::parse.map(Value::from),
            Double::parse.map(Value::from),
            Map::parse.map(Value::from),
            Null::parse.map(Value::from),
            Number::parse.map(Value::from),
            Set::parse.map(Value::from),
            SimpleError::parse.map(Value::from),
            SimpleString::parse.map(Value::from),
            VerbatimString::parse.map(Value::from),
        ))
        .parse(input)
    }
}

impl TryFrom<&Value> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: &Value) -> anyhow::Result<Bytes> {
        match input {
            Value::Array(inner) => Bytes::try_from(inner),
            Value::BigNumber(inner) => Bytes::try_from(inner),
            Value::BlobError(inner) => Bytes::try_from(inner),
            Value::BlobString(inner) => Bytes::try_from(inner),
            Value::Boolean(inner) => Bytes::try_from(inner),
            Value::Double(inner) => Bytes::try_from(inner),
            Value::Map(inner) => Bytes::try_from(inner),
            Value::Null(inner) => Bytes::try_from(inner),
            Value::Number(inner) => Bytes::try_from(inner),
            Value::Set(inner) => Bytes::try_from(inner),
            Value::SimpleError(inner) => Bytes::try_from(inner),
            Value::SimpleString(inner) => Bytes::try_from(inner),
            Value::VerbatimString(inner) => Bytes::try_from(inner),
        }
    }
}

impl TryFrom<Value> for Bytes {
    type Error = anyhow::Error;

    fn try_from(input: Value) -> anyhow::Result<Bytes> {
        Bytes::try_from(&input)
    }
}
