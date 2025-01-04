//! Read data from `&[u8]` with little endian.

use std::borrow::Cow;

use super::encoding::decode_string;
use crate::error::{FvpError, FvpResult};

pub trait FvpBuffer {
  fn fread<'a, N: FvpRead<'a>>(&'a self, offset: usize) -> FvpResult<N>;
}

impl FvpBuffer for [u8] {
  fn fread<'a, N: FvpRead<'a>>(&'a self, offset: usize) -> FvpResult<N> {
    N::from_buffer(&self[offset..])
  }
}

pub trait FvpRead<'a> {
  fn from_buffer(buffer: &'a [u8]) -> FvpResult<Self>
  where
    Self: Sized;
}

impl FvpRead<'_> for u16 {
  fn from_buffer(buffer: &[u8]) -> FvpResult<Self> {
    match buffer[..2].try_into() {
      Ok(data) => Ok(u16::from_le_bytes(data)),
      Err(_) => Err(FvpError::OffsetTooLarge),
    }
  }
}

impl FvpRead<'_> for u32 {
  fn from_buffer(buffer: &[u8]) -> FvpResult<Self> {
    match buffer[..4].try_into() {
      Ok(data) => Ok(u32::from_le_bytes(data)),
      Err(_) => Err(FvpError::OffsetTooLarge),
    }
  }
}

impl<'a> FvpRead<'a> for Cow<'a, str> {
  fn from_buffer(buffer: &'a [u8]) -> FvpResult<Self> {
    match buffer.iter().position(|b| *b == 0) {
      Some(end) => decode_string(&buffer[..end]),
      None => Err(FvpError::OffsetTooLarge),
    }
  }
}
