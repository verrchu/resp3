pub mod prop;

use std::str::FromStr;

use bytes::Bytes;
use num_bigint::BigInt;

use super::{complete::primitive::double, *};

#[test]
fn test_basic_array() {
    assert_eq!(
        Value::parse(&b"*1\r\n+test\r\n"[..]),
        Ok((
            &b""[..],
            Value::Array(Array::from([Value::SimpleString(SimpleString::from(
                "test"
            ))]))
        ))
    );
}

#[test]
fn test_basic_boolean() {
    assert_eq!(
        Value::parse(&b"#f\r\n"[..]),
        Ok((&b""[..], Value::Boolean(Boolean(false))))
    );
}

#[test]
fn test_basic_big_number() {
    let raw = ['1'; 100].into_iter().collect::<String>();

    assert_eq!(
        Value::parse(format!("({raw}\r\n").as_bytes()),
        Ok((
            &b""[..],
            Value::BigNumber(BigNumber(BigInt::from_str(&raw).unwrap()))
        ))
    );
}

#[test]
fn test_basic_blob_error() {
    assert_eq!(
        Value::parse(&b"!10\r\nERR reason\r\n"[..]),
        Ok((
            &b""[..],
            Value::BlobError(BlobError::new("ERR", b"reason".to_vec()))
        ))
    );
}

#[test]
fn test_basic_blob_string() {
    assert_eq!(
        Value::parse(&b"$4\r\ntest\r\n"[..]),
        Ok((
            &b""[..],
            Value::BlobString(BlobString(Bytes::from(b"test".to_vec())))
        ))
    );
}

#[test]
fn test_basic_double() {
    assert_eq!(
        Value::parse(&b",-inf\r\n"[..]),
        Ok((&b""[..], Value::Double(Double::Inf(double::Sign::Minus))))
    );
}

#[test]
fn test_basic_number() {
    assert_eq!(
        Value::parse(&b":0\r\n"[..]),
        Ok((&b""[..], Value::Number(Number(0))))
    );
}

#[test]
fn test_basic_null() {
    assert_eq!(Value::parse(&b"_\r\n"[..]), Ok((&b""[..], Value::Null)));
}

#[test]
fn test_basic_simple_error() {
    assert_eq!(
        Value::parse(&b"-ERR reason\r\n"[..]),
        Ok((
            &b""[..],
            Value::SimpleError(SimpleError::new("ERR", "reason"))
        ))
    );
}

#[test]
fn test_basic_simple_string() {
    assert_eq!(
        Value::parse(&b"+test\r\n"[..]),
        Ok((&b""[..], Value::SimpleString(SimpleString::from("test"))))
    );
}

#[test]
fn test_basic_verbatim_string() {
    assert_eq!(
        Value::parse(&b"=10\r\ntxt:123456\r\n"[..]),
        Ok((
            &b""[..],
            Value::VerbatimString(VerbatimString::Txt(Bytes::from(b"123456".to_vec())))
        ))
    );
}
