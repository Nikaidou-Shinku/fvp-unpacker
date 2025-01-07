use thiserror::Error;

#[derive(Debug, Error)]
pub enum FvpError {
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error("Offset is too large")]
  OffsetTooLarge,

  #[error("Can not decode bytes to string")]
  CannotDecodeString,

  #[error("Can not encode string to bytes")]
  CannotEncodeString,

  #[error("Detected string encoding mismatch")]
  StringEncodingMismatch,

  #[error("Format signature mismatch (for {format}, expected {expected:x?}, but found {found:x?})")]
  FormatMismatch {
    format: &'static str,
    expected: &'static [u8; 4],
    found: Box<[u8]>,
  },

  #[error("Decompressed data length mismatch (expected {expected}, but found {found})")]
  DecompressLengthMismatch { expected: usize, found: usize },

  #[error(transparent)]
  ImageEncoding(#[from] png::EncodingError),

  #[error("Image width")]
  ImageWidthMismatch { expected: u16, found: usize },

  #[error("Image width")]
  ImageHeightMismatch { expected: u16, found: usize },

  #[error("Image width")]
  ImageOffsetMismatch {
    expected: (u16, u16),
    found: (u16, u16),
  },
}

pub type FvpResult<T> = Result<T, FvpError>;
