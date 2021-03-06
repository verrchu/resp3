pub mod prop;

use super::*;
use crate::value::{complete::primitive::double, *};
use num_bigint::BigInt;

#[test]
fn test_empty() {
    assert_eq!(
        Array::parse(&b"*0\r\n"[..]),
        Ok((&b""[..], Array::from([])))
    );
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
            Array::from([
                Value::from(BigNumber::from(BigInt::from_str("12345").unwrap())),
                Value::from(BlobError::new("ERR", b"reason".to_vec())),
                Value::from(BlobString::from(b"test".to_vec())),
                Value::from(Boolean::from(false)),
                Value::from(Double::inf(double::Sign::Minus)),
                Value::from(Null::default()),
                Value::from(Number::from(1234)),
                Value::from(SimpleError::new("ERR", "reason")),
                Value::from(SimpleString::from("test")),
                Value::from(VerbatimString::txt(b"test".to_vec())),
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
                    Value::from(Boolean::from(false)),
                    Value::from(Number::from(-1))
                ])),
            ])
        ))
    );
}
