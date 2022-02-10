pub mod prop;

use std::str::FromStr;

use super::*;
use crate::value::{complete::primitive::double, *};
use num_bigint::BigInt;

#[test]
fn test_empty() {
    assert_eq!(Map::parse(&b"%0\r\n"[..]), Ok((&b""[..], Map::from([]))));
}

#[test]
fn test_heterogenous_keys_simple() {
    let raw = "\
                   %10\r\n\
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
        Map::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Map::from([
                (
                    Value::from(BigNumber(BigInt::from_str("12345").unwrap())),
                    Value::from(Null::default())
                ),
                (
                    Value::from(BlobError::new("ERR", b"reason".to_vec())),
                    Value::from(Null::default())
                ),
                (
                    Value::from(BlobString::from(b"test".to_vec())),
                    Value::from(Null::default())
                ),
                (
                    Value::from(Boolean::from(false)),
                    Value::from(Null::default())
                ),
                (
                    Value::from(Double::Inf(double::Sign::Minus)),
                    Value::from(Null::default())
                ),
                (Value::from(Null::default()), Value::from(Null::default())),
                (Value::from(Number(1234)), Value::from(Null::default())),
                (
                    Value::from(SimpleError::new("ERR", "reason")),
                    Value::from(Null::default())
                ),
                (
                    Value::from(SimpleString::from("test")),
                    Value::from(Null::default())
                ),
                (
                    Value::from(VerbatimString::txt(b"test".to_vec())),
                    Value::from(Null::default())
                ),
            ])
        ))
    );
}

#[test]
fn test_heterogenous_value_simple() {
    let raw = "\
                   %10\r\n\
                   :0\r\n(12345\r\n\
                   :1\r\n!10\r\nERR reason\r\n\
                   :2\r\n$4\r\ntest\r\n\
                   :3\r\n#f\r\n\
                   :4\r\n,-inf\r\n\
                   :5\r\n_\r\n\
                   :6\r\n:1234\r\n\
                   :7\r\n-ERR reason\r\n\
                   :8\r\n+test\r\n\
                   :9\r\n=8\r\ntxt:test\r\n\
                   ";

    assert_eq!(
        Map::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Map::from([
                (
                    Value::from(Number::from(0)),
                    Value::from(BigNumber(BigInt::from_str("12345").unwrap()))
                ),
                (
                    Value::from(Number::from(1)),
                    Value::from(BlobError::new("ERR", b"reason".to_vec()))
                ),
                (
                    Value::from(Number::from(2)),
                    Value::from(BlobString::from(b"test".to_vec()))
                ),
                (
                    Value::from(Number::from(3)),
                    Value::from(Boolean::from(false))
                ),
                (
                    Value::from(Number::from(4)),
                    Value::from(Double::Inf(double::Sign::Minus))
                ),
                (Value::from(Number::from(5)), Value::from(Null::default())),
                (Value::from(Number::from(6)), Value::from(Number(1234))),
                (
                    Value::from(Number::from(7)),
                    Value::from(SimpleError::new("ERR", "reason"))
                ),
                (
                    Value::from(Number::from(8)),
                    Value::from(SimpleString::from("test"))
                ),
                (
                    Value::from(Number::from(9)),
                    Value::from(VerbatimString::txt(b"test".to_vec()))
                ),
            ])
        ))
    );
}

#[test]
fn test_nested_map_key() {
    let raw = "\
            %1\r\n\
                %2\r\n\
                    :1\r\n#f\r\n\
                    :2\r\n#t\r\n\
                _\r\n";

    assert_eq!(
        Map::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Map::from([(
                Value::Map(Map::from([
                    (
                        Value::from(Number::from(1)),
                        Value::from(Boolean::from(false))
                    ),
                    (
                        Value::from(Number::from(2)),
                        Value::from(Boolean::from(true))
                    ),
                ])),
                Value::from(Null::default())
            )])
        ))
    );
}

#[test]
fn test_nested_map_value() {
    let raw = "\
            %1\r\n\
                _\r\n\
                %2\r\n\
                    :1\r\n#f\r\n\
                    :2\r\n#t\r\n\
                  ";

    assert_eq!(
        Map::parse(raw.as_bytes()),
        Ok((
            &b""[..],
            Map::from([(
                Value::from(Null::default()),
                Value::Map(Map::from([
                    (
                        Value::from(Number::from(1)),
                        Value::from(Boolean::from(false))
                    ),
                    (
                        Value::from(Number::from(2)),
                        Value::from(Boolean::from(true))
                    ),
                ]))
            )])
        ))
    );
}
