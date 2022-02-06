use std::str::{self, FromStr};

use super::DELIMITER;

use bytes::Bytes;
use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    error::{Error, ErrorKind},
    sequence::{preceded, terminated},
    Err, IResult, Parser,
};

#[derive(Debug)]
pub struct BlobString(Bytes);

impl BlobString {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, len) = terminated(preceded(tag("$"), digit1), tag(DELIMITER))
            .parse(input)
            .and_then(|(i, o)| {
                let o = str::from_utf8(o)
                    .map_err(|_| Err::Error(Error::new(input, ErrorKind::Digit)))?;
                let o = u64::from_str(o)
                    .map_err(|_| Err::Error(Error::new(input, ErrorKind::Digit)))?;

                Ok((i, o))
            })?;

        terminated(take(len), tag(DELIMITER))
            .map(|bytes: &[u8]| BlobString(Bytes::from(bytes.to_vec())))
            .parse(input)
    }
}
