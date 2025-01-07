use std::io::{Read, Write};

use bytemuck::AnyBitPattern;
use flate2::bufread::ZlibDecoder;
use imgref::ImgVec;
use png::ColorType;
use rgb::{Bgr, Bgra, Gray, Rgb, Rgba};

use crate::{
  error::{FvpError, FvpResult},
  utils::sread::FvpBuffer,
};

pub struct FvpHzcEntry<Pixel> {
  offset: (u16, u16),
  data: ImgVec<Pixel>,
  // TODO: replace `Vec<Pixel>` with `Box<[Pixel]>`
  // data: Img<Box<[Pixel]>>,
}

impl<Pixel> FvpHzcEntry<Pixel> {
  fn new(width: u16, height: u16, offset: (u16, u16), buf: impl Into<Vec<Pixel>>) -> Self {
    fn new_inner<Pixel>(
      width: u16,
      height: u16,
      offset: (u16, u16),
      buf: Vec<Pixel>,
    ) -> FvpHzcEntry<Pixel> {
      FvpHzcEntry {
        offset,
        data: ImgVec::new(buf, width as usize, height as usize),
      }
    }

    let buf = buf.into();
    new_inner(width, height, offset, buf)
  }
}

fn write_to_png_inner<W: Write>(
  writer: W,
  width: u32,
  height: u32,
  color: ColorType,
  data: &[u8],
) -> FvpResult<()> {
  let mut encoder = png::Encoder::new(writer, width, height);
  encoder.set_color(color);
  let mut writer = encoder.write_header()?;
  writer.write_image_data(data)?;
  Ok(())
}

impl FvpHzcEntry<Bgr<u8>> {
  pub fn write_to_png<W: Write>(&self, writer: W) -> FvpResult<()> {
    let data: Box<[Rgb<u8>]> = self.data.pixels().map(Into::into).collect();
    let data = bytemuck::cast_slice_box(data);
    write_to_png_inner(
      writer,
      self.data.width() as u32,
      self.data.height() as u32,
      ColorType::Rgb,
      &data,
    )?;
    Ok(())
  }
}

impl FvpHzcEntry<Bgra<u8>> {
  pub fn write_to_png<W: Write>(&self, writer: W) -> FvpResult<()> {
    let data: Box<[Rgba<u8>]> = self.data.pixels().map(Into::into).collect();
    let data = bytemuck::cast_slice_box(data);
    write_to_png_inner(
      writer,
      self.data.width() as u32,
      self.data.height() as u32,
      ColorType::Rgba,
      &data,
    )?;
    Ok(())
  }
}

impl FvpHzcEntry<Gray<u8>> {
  pub fn write_to_png<W: Write>(&self, writer: W) -> FvpResult<()> {
    let data = bytemuck::cast_slice(self.data.buf());
    write_to_png_inner(
      writer,
      self.data.width() as u32,
      self.data.height() as u32,
      ColorType::Grayscale,
      &data,
    )?;
    Ok(())
  }
}

pub struct FvpHzc<Pixel> {
  width: u16,
  height: u16,
  offset: (u16, u16),
  entries: Vec<FvpHzcEntry<Pixel>>,
}

impl<Pixel> FvpHzc<Pixel> {
  fn new(width: u16, height: u16, offset: (u16, u16)) -> Self {
    Self {
      width,
      height,
      offset,
      entries: Vec::new(),
    }
  }

  fn add_entry(&mut self, entry: impl Into<FvpHzcEntry<Pixel>>) -> FvpResult<&mut Self> {
    fn add_entry_inner<Pixel>(
      this: &mut FvpHzc<Pixel>,
      entry: FvpHzcEntry<Pixel>,
    ) -> FvpResult<&mut FvpHzc<Pixel>> {
      if entry.data.width() != this.width as usize {
        return Err(FvpError::ImageWidthMismatch {
          expected: this.width,
          found: entry.data.width(),
        });
      }

      if entry.data.height() != this.height as usize {
        return Err(FvpError::ImageHeightMismatch {
          expected: this.height,
          found: entry.data.height(),
        });
      }

      if entry.offset != this.offset {
        return Err(FvpError::ImageOffsetMismatch {
          expected: this.offset,
          found: entry.offset,
        });
      }

      this.entries.push(entry);
      Ok(this)
    }

    let entry = entry.into();
    add_entry_inner(self, entry)
  }

  pub fn entries(&self) -> &[FvpHzcEntry<Pixel>] {
    &self.entries
  }
}

pub enum DynamicFvpHzc {
  Bgr(FvpHzc<Bgr<u8>>),
  Bgra(FvpHzc<Bgra<u8>>),
  Gray(FvpHzc<Gray<u8>>),
  // TODO: image with only black and white pixels
  // Binary(FvpHzc<u8>),
}

impl DynamicFvpHzc {
  // TODO: zero-copy
  pub fn parse(src: impl AsRef<[u8]>) -> FvpResult<Self> {
    fn parse_inner(src: &[u8]) -> FvpResult<DynamicFvpHzc> {
      let signature: u32 = src.sread(0)?;

      if signature != u32::from_le_bytes(*b"hzc1") {
        return Err(FvpError::FormatMismatch {
          format: "Hzc file",
          expected: b"hzc1",
          found: Box::from(&src[..4]),
        });
      }

      let unpacked_size = src.sread::<u32>(4)? as usize;
      let header_size = src.sread::<u32>(8)? as usize;

      let data_index = 12 + header_size;

      struct FvpHzcHeader {
        // signature: u32,
        // unknown1: u16,
        color: u16,
        width: u16,
        height: u16,
        offset_x: u16,
        offset_y: u16,
        // unknown2: u32,
        count: u32,
        // unknown3: u64,
      }

      impl FvpHzcHeader {
        fn parse(src: &[u8]) -> FvpResult<Self> {
          let signature: u32 = src.sread(0)?;

          if signature != u32::from_le_bytes(*b"NVSG") {
            return Err(FvpError::FormatMismatch {
              format: "Hzc header",
              expected: b"NVSG",
              found: Box::from(&src[..4]),
            });
          }

          let color: u16 = src.sread(6)?;
          let width: u16 = src.sread(8)?;
          let height: u16 = src.sread(10)?;
          let offset_x: u16 = src.sread(12)?;
          let offset_y: u16 = src.sread(14)?;
          let count: u32 = match src.sread(20)? {
            0 => 1,
            x => x,
          };

          Ok(Self {
            color,
            width,
            height,
            offset_x,
            offset_y,
            count,
          })
        }
      }

      let header = FvpHzcHeader::parse(&src[12..data_index])?;

      let data = {
        let mut z = ZlibDecoder::new(&src[data_index..]);
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

      fn create_hzc_archive<Pixel: AnyBitPattern>(
        header: FvpHzcHeader,
        data: Box<[u8]>,
      ) -> FvpResult<FvpHzc<Pixel>> {
        let data = bytemuck::cast_slice_box(data);
        let size = data.len() / header.count as usize;

        let mut archive = FvpHzc::new(
          header.width,
          header.height,
          (header.offset_x, header.offset_y),
        );

        for i in 0..header.count as usize {
          archive.add_entry(FvpHzcEntry::new(
            header.width,
            header.height,
            (header.offset_x, header.offset_y),
            &data[(i * size)..(i * size + size)],
          ))?;
        }

        Ok(archive)
      }

      Ok(match header.color {
        0 => DynamicFvpHzc::Bgr(create_hzc_archive(header, data)?),
        1 | 2 => DynamicFvpHzc::Bgra(create_hzc_archive(header, data)?),
        3 => DynamicFvpHzc::Gray(create_hzc_archive(header, data)?),
        4 => unimplemented!(),
        _ => unreachable!(),
      })
    }

    let src = src.as_ref();
    parse_inner(src)
  }
}
