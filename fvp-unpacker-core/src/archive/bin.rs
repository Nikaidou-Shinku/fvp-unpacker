use std::{borrow::Cow, io::Write};

use crate::{
  error::FvpResult,
  utils::{encoding::encode_string, sread::FvpBuffer},
};

pub struct FvpBinEntry {
  filename: String,
  data: Box<[u8]>,
}

impl FvpBinEntry {
  pub fn new(filename: impl Into<String>, data: impl Into<Box<[u8]>>) -> Self {
    fn new_inner(filename: String, data: Box<[u8]>) -> FvpBinEntry {
      FvpBinEntry { filename, data }
    }

    let filename = filename.into();
    let data = data.into();
    new_inner(filename, data)
  }

  pub fn filename(&self) -> &str {
    &self.filename
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }
}

#[derive(Default)]
pub struct FvpBin(Vec<FvpBinEntry>);

impl FvpBin {
  pub fn new(entries: impl Into<Vec<FvpBinEntry>>) -> Self {
    fn new_inner(entries: Vec<FvpBinEntry>) -> FvpBin {
      FvpBin(entries)
    }

    let entries = entries.into();
    new_inner(entries)
  }

  pub fn add_entry(&mut self, entry: impl Into<FvpBinEntry>) -> &mut Self {
    fn add_entry_inner(this: &mut FvpBin, entry: FvpBinEntry) -> &mut FvpBin {
      this.0.push(entry);
      this
    }

    let entry = entry.into();
    add_entry_inner(self, entry)
  }

  pub fn entries(&self) -> &[FvpBinEntry] {
    &self.0
  }

  // TODO: zero-copy
  pub fn parse(src: impl AsRef<[u8]>) -> FvpResult<Self> {
    fn parse_inner(src: &[u8]) -> FvpResult<FvpBin> {
      let count: u32 = src.sread(0)?;
      // TODO: maybe check this to make sure the archive is not corrupted
      let _name_index_size: u32 = src.sread(4)?;

      let names_base = count as usize * 12 + 8;

      let entries = (0..count)
        .map(|i| {
          let index_offset = i as usize * 12 + 8;

          let filename_offset: u32 = src.sread(index_offset)?;
          let filename: Cow<str> = src.sread(names_base + filename_offset as usize)?;

          let offset = src.sread::<u32>(index_offset + 4)? as usize;
          let size = src.sread::<u32>(index_offset + 8)? as usize;

          Ok(FvpBinEntry::new(filename, &src[offset..(offset + size)]))
        })
        .collect::<FvpResult<_>>()?;

      Ok(FvpBin(entries))
    }

    let src = src.as_ref();
    parse_inner(src)
  }

  pub fn write<W: Write>(&self, mut writer: W) -> FvpResult<()> {
    let count = self.0.len() as u32;

    let filenames = self
      .0
      .iter()
      .map(|entry| encode_string(entry.filename()))
      .collect::<FvpResult<Box<_>>>()?;

    let name_index_size = filenames
      .iter()
      .map(|filename| filename.len() + 1)
      .sum::<usize>() as u32;

    writer.write_all(&count.to_le_bytes())?;
    writer.write_all(&name_index_size.to_le_bytes())?;

    let mut name_offset: u32 = 0;
    let mut data_offset: u32 = count * 12 + 8 + name_index_size;

    for (entry, filename) in self.0.iter().zip(filenames.iter()) {
      let data_size = entry.data().len() as u32;

      writer.write_all(&name_offset.to_le_bytes())?;
      writer.write_all(&data_offset.to_le_bytes())?;
      writer.write_all(&data_size.to_le_bytes())?;

      name_offset += (filename.len() + 1) as u32;
      data_offset += data_size;
    }

    for filename in filenames {
      writer.write_all(&filename)?;
      writer.write_all(&[0])?;
    }

    for entry in &self.0 {
      writer.write_all(entry.data())?;
    }

    Ok(())
  }
}
