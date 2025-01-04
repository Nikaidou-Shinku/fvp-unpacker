use std::io::Read;

use flate2::bufread::ZlibDecoder;

use crate::{
  error::{FvpError, FvpResult},
  utils::fread::{FvpBuffer, FvpRead},
};

#[derive(Clone)]
pub struct FvpHzcHeader {
  // 4 bytes: signature
  // 2 bytes: unknown
  r#type: u16,
  width: u16,
  height: u16,
  offset_x: u16,
  offset_y: u16,
  // 4 bytes: unknown
  count: u32,
  // 8 bytes: unknown
}

impl FvpRead<'_> for FvpHzcHeader {
  fn from_buffer(buffer: &[u8]) -> FvpResult<Self> {
    if &buffer[..4] != b"NVSG" {
      return Err(FvpError::FormatMismatch {
        format: "Hzc header",
        expected: b"NVSG",
        found: Box::from(&buffer[..4]),
      });
    }

    let r#type: u16 = buffer.fread(6)?;
    let width: u16 = buffer.fread(8)?;
    let height: u16 = buffer.fread(10)?;
    let offset_x: u16 = buffer.fread(12)?;
    let offset_y: u16 = buffer.fread(14)?;
    let count: u32 = match buffer.fread(20)? {
      0 => 1,
      x => x,
    };

    Ok(Self {
      r#type,
      width,
      height,
      offset_x,
      offset_y,
      count,
    })
  }
}

pub struct FvpHzc {
  header: FvpHzcHeader,
  data: Box<[u8]>,
}

impl FvpRead<'_> for FvpHzc {
  fn from_buffer(buffer: &[u8]) -> FvpResult<Self> {
    if &buffer[..4] != b"hzc1" {
      return Err(FvpError::FormatMismatch {
        format: "Hzc file",
        expected: b"hzc1",
        found: Box::from(&buffer[..4]),
      });
    }

    let unpacked_size: u32 = buffer.fread(4)?;
    let unpacked_size = unpacked_size as usize;
    let header_size: u32 = buffer.fread(8)?;

    let index_data = 12 + header_size as usize;

    let header: FvpHzcHeader = buffer[12..index_data].fread(0)?;

    let data = {
      let mut z = ZlibDecoder::new(&buffer[index_data..]);
      let mut res = Vec::with_capacity(unpacked_size);
      z.read_to_end(&mut res)?;
      res.into_boxed_slice()
    };

    if data.len() != unpacked_size {
      return Err(FvpError::DecompressLengthMismatch {
        expected: unpacked_size,
        found: data.len(),
      });
    }

    Ok(Self { header, data })
  }
}

impl FvpHzc {
  pub fn entries(&self) -> FvpHzcIter {
    FvpHzcIter {
      data: self,
      current: 0,
    }
  }
}

pub struct FvpHzcIter<'a> {
  data: &'a FvpHzc,
  current: usize,
}

impl<'a> Iterator for FvpHzcIter<'a> {
  type Item = FvpHzcEntry<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let total_size = self.data.data.len();

    if self.current >= total_size {
      return None;
    }

    let size = total_size / self.data.header.count as usize;

    let res = Some(FvpHzcEntry {
      header: self.data.header.clone(),
      data: &self.data.data[self.current..(self.current + size)],
    });

    self.current += size;

    res
  }
}

pub struct FvpHzcEntry<'a> {
  header: FvpHzcHeader,
  data: &'a [u8],
}

impl<'a> FvpHzcEntry<'a> {
  pub fn offset(&self) -> (u16, u16) {
    (self.header.offset_x, self.header.offset_y)
  }

  pub fn write_to_png<W: std::io::Write>(&self, writer: W) -> FvpResult<()> {
    // TODO: maybe zero-copy here
    let (color_type, data) = match self.header.r#type {
      0 => {
        let data: Box<_> = self
          .data
          .array_chunks()
          .flat_map(|&[b, g, r]| [r, g, b])
          .collect();

        (png::ColorType::Rgb, data)
      }
      1 | 2 => {
        let data: Box<_> = self
          .data
          .array_chunks()
          .flat_map(|&[b, g, r, a]| [r, g, b, a])
          .collect();

        (png::ColorType::Rgba, data)
      }
      3 => (png::ColorType::Grayscale, self.data.into()),
      4 => unimplemented!(), // FIXME: complete this
      _ => unreachable!(),
    };

    let mut encoder =
      png::Encoder::new(writer, self.header.width.into(), self.header.height.into());
    encoder.set_color(color_type);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data)?;

    Ok(())
  }
}
