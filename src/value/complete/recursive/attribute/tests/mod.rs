pub mod prop;

use std::str::FromStr;

use super::*;
use crate::value::{complete::primitive::double, *};
use num_bigint::BigInt;

#[test]
fn test_empty() {
    assert_eq!(
        Attribute::parse(&b"|0\r\n"[..]),
        Ok((&b""[..], Attribute::from([])))
    );
}

#[test]
fn test_heterogenous_keys_simple() {
    let raw = "\
                   |10\r\n\
                   (12345\r\n_\r\n\
                   !10\r\nERR reason\r\n_\r\n\
                   $4\r\ntest\r\n_\r\n\
                   #f\r\n_\r\n\
                   ,-inf\r\n_\r\n\
                   _\r\n_\r\n\
                   :1234\r\n_\r\n\
                   -ERR reason\r\n_\r\n\
                   +test\r\n_\r\n\
                   =8\r\ntxt:test\r\n_\r\n\
                   ";

    assert_eq!(
        Attribute::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Attribute::from([
                (
                    Value::BigNumber(BigNumber(BigInt::from_str("12345").unwrap())),
                    Value::Null
                ),
                (
                    Value::BlobError(BlobError::new("ERR", b"reason".to_vec())),
                    Value::Null
                ),
                (
                    Value::BlobString(BlobString::from(b"test".to_vec())),
                    Value::Null
                ),
                (Value::Boolean(Boolean(false)), Value::Null),
                (Value::Double(Double::Inf(double::Sign::Minus)), Value::Null),
                (Value::Null, Value::Null),
                (Value::Number(Number(1234)), Value::Null),
                (
                    Value::SimpleError(SimpleError::new("ERR", "reason")),
                    Value::Null
                ),
                (Value::SimpleString(SimpleString::from("test")), Value::Null),
                (
                    Value::VerbatimString(VerbatimString::txt(b"test".to_vec())),
                    Value::Null
                ),
            ])
        ))
    );
}
