use std::borrow::Cow;

use crate::{
  error::FvpResult,
  utils::fread::{FvpBuffer, FvpRead},
};

pub struct FvpBinEntry<'a> {
  filename: Cow<'a, str>,
  offset: u32,
  size: u32,
  buffer: &'a [u8],
}

impl FvpBinEntry<'_> {
  pub fn filename(&self) -> &Cow<str> {
    &self.filename
  }

  pub fn data(&self) -> &[u8] {
    let start = self.offset as usize;
    let end = (self.offset + self.size) as usize;
    &self.buffer[start..end]
  }
}

pub struct FvpBin<'a>(Box<[FvpBinEntry<'a>]>);

impl<'a> FvpRead<'a> for FvpBin<'a> {
  fn from_buffer(buffer: &'a [u8]) -> FvpResult<Self> {
    let count: u32 = buffer.fread(0)?;

    let names_base = count as usize * 12 + 8;

    let entries = (0..count)
      .map(|i| {
        let index_offset = i as usize * 12 + 8;

        let filename_offset: u32 = buffer.fread(index_offset)?;
        let filename: Cow<str> = buffer.fread(names_base + filename_offset as usize)?;

        let offset: u32 = buffer.fread(index_offset + 4)?;
        let size: u32 = buffer.fread(index_offset + 8)?;

        Ok(FvpBinEntry {
          filename,
          offset,
          size,
          buffer,
        })
      })
      .collect::<FvpResult<_>>()?;

    Ok(Self(entries))
  }
}

impl FvpBin<'_> {
  pub fn entries(&self) -> &[FvpBinEntry] {
    &self.0
  }
}
