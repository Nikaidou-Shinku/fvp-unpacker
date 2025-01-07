use std::borrow::Cow;

use encoding_rs::SHIFT_JIS;

use crate::error::{FvpError, FvpResult};

pub fn decode_string(bytes: &[u8]) -> FvpResult<Cow<str>> {
  let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(bytes);

  if had_errors {
    return Err(FvpError::CannotDecodeString);
  }

  if encoding_used != SHIFT_JIS {
    return Err(FvpError::StringEncodingMismatch);
  }

  Ok(cow)
}

pub fn encode_string(string: &str) -> FvpResult<Cow<[u8]>> {
  let (cow, encoding_used, had_errors) = SHIFT_JIS.encode(string);

  if had_errors {
    return Err(FvpError::CannotEncodeString);
  }

  if encoding_used != SHIFT_JIS {
    return Err(FvpError::StringEncodingMismatch);
  }

  Ok(cow)
}
