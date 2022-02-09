pub mod prop;

use super::*;
use crate::value::{complete::primitive::double, *};
use num_bigint::BigInt;

#[test]
fn test_empty() {
    assert_eq!(Array::parse(&b"*0\r\n"[..]), Ok((&b""[..], Array(vec![]))));
}

#[test]
fn test_heterogenous_simple() {
    let raw = "\
                   *10\r\n\
                   (12345\r\n\
                   !10\r\nERR reason\r\n\
                   $4\r\ntest\r\n\
                   #f\r\n\
                   ,-inf\r\n\
                   _\r\n\
                   :1234\r\n\
                   -ERR reason\r\n\
                   +test\r\n\
                   =8\r\ntxt:test\r\n\
                   ";

    assert_eq!(
        Array::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Array(vec![
                Value::BigNumber(BigNumber(BigInt::from_str("12345").unwrap())),
                Value::BlobError(BlobError::new("ERR", b"reason".to_vec())),
                Value::BlobString(BlobString::from(b"test".to_vec())),
                Value::Boolean(Boolean(false)),
                Value::Double(Double::Inf(double::Sign::Minus)),
                Value::Null,
                Value::Number(Number(1234)),
                Value::SimpleError(SimpleError::new("ERR", "reason")),
                Value::SimpleString(SimpleString::from("test")),
                Value::VerbatimString(VerbatimString::txt(b"test".to_vec())),
            ])
        ))
    );
}

#[test]
fn test_nested_array() {
    let raw = "\
                   *2\r\n\
                   *1\r\n+test\r\n\
                   *2\r\n#f\r\n:-1\r\n\
                   ";

    assert_eq!(
        Array::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Array::from([
                Value::Array(Array::from([Value::SimpleString(SimpleString::from(
                    "test"
                ))])),
                Value::Array(Array::from([
                    Value::Boolean(Boolean(false)),
                    Value::Number(Number(-1))
                ])),
            ])
        ))
    );
}
