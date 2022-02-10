pub mod prop;

use super::*;

#[test]
fn test_positive_number() {
    let str = ['1'; 100].into_iter().collect::<String>();
    let raw = format!("({str}\r\n");

    let (i, o) = BigNumber::parse(raw.as_bytes()).unwrap();

    assert!(i.is_empty());
    assert_eq!(o.val().to_string(), str);
}

#[test]
fn test_negative_number() {
    let str = ['1'; 100].into_iter().collect::<String>();
    let str = format!("-{str}");

    let raw = format!("({str}\r\n");

    let (i, o) = BigNumber::parse(raw.as_bytes()).unwrap();

    assert!(i.is_empty());
    assert_eq!(o.val().to_string(), str);
}
