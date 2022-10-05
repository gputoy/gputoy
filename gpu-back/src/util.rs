use std::array::TryFromSliceError;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid encoded id")]
    InvalidEncodedId(#[from] base64::DecodeError),
    #[error("Invalid id length")]
    InvalidIdLength(#[from] TryFromSliceError),
}

pub fn to_base64(uuid: &Uuid) -> String {
    let buf = uuid.as_bytes();
    base64::encode_config(buf, base64::URL_SAFE_NO_PAD)
}

pub fn from_base64(encoded: &String) -> Result<Uuid, Error> {
    base64::decode_config(encoded, base64::URL_SAFE_NO_PAD)?
        .as_slice()
        .try_into()
        .map(Uuid::from_bytes)
        .map_err(Error::InvalidIdLength)
}
