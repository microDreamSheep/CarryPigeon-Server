use serde::Serialize;

use crate::error::Error;
use crate::header::Header;

#[derive(Debug, Clone, Copy)]
pub struct EncodingKey {}

impl EncodingKey {}

/// Encode the header and claims given and sign the payload using the algorithm from the header and the key.
pub fn encode<T: Serialize>(
    header: &Header,
    claims: T,
    key: &EncodingKey,
) -> Result<String, Error> {
    return Err(Error::Missing);
}
