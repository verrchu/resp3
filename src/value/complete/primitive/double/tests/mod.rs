pub mod prop;

use super::*;

#[test]
fn test_positive_number() {
    assert_eq!(
        Double::parse(&b",1.234\r\n"[..]),
        Ok((
            &b""[..],
            Double::from_parts(Parts {
                sign: Sign::Plus,
                int: 1,
                frac: Some(234)
            })
            .unwrap()
        ))
    );
}

#[test]
fn test_negative_number() {
    assert_eq!(
        Double::parse(&b",-1.234\r\n"[..]),
        Ok((
            &b""[..],
            Double::from_parts(Parts {
                sign: Sign::Minus,
                int: 1,
                frac: Some(234)
            })
            .unwrap()
        ))
    );
}

#[test]
fn test_positive_infinity() {
    assert_eq!(
        Double::parse(&b",inf\r\n"[..]),
        Ok((&b""[..], Double::inf(Sign::Plus)))
    );
}

#[test]
fn test_negative_infinity() {
    assert_eq!(
        Double::parse(&b",-inf\r\n"[..]),
        Ok((&b""[..], Double::inf(Sign::Minus)))
    );
}
