use std::{
  fs::{self, File},
  io::BufWriter,
  path::PathBuf,
};

use anyhow::Result;
use clap::Args;
use fvp_unpacker_core::prelude::*;
use memmap2::Mmap;
use rayon::prelude::*;

#[derive(Args)]
pub struct UnpackArgs {
  /// Input file path
  #[arg(short, long)]
  input: PathBuf,

  /// Output directory path
  #[arg(short, long, default_value = "./output")]
  output: PathBuf,
}

pub fn unpack(args: &UnpackArgs) -> Result<()> {
  if !args.output.is_dir() {
    fs::create_dir_all(&args.output)?;
  }

  let input_file = File::open(&args.input)?;
  // SAFETY: it's not my fault :(
  let content = unsafe { Mmap::map(&input_file) }?;

  // TODO: handle other formats
  let arc = FvpBin::parse(content)?;

  arc
    .entries()
    .par_iter()
    .map(|entry| {
      let filename = entry.filename();

      // TODO: handle other file formats
      match DynamicFvpHzc::parse(entry.data())? {
        DynamicFvpHzc::Bgr(hzc) => {
          for (i, img) in hzc.entries().iter().enumerate() {
            let output_path = args.output.join(format!("{filename}-{i}.png"));
            let output_file = File::create(output_path)?;
            img.write_to_png(BufWriter::new(output_file))?;
          }
        }
        DynamicFvpHzc::Bgra(hzc) => {
          for (i, img) in hzc.entries().iter().enumerate() {
            let output_path = args.output.join(format!("{filename}-{i}.png"));
            let output_file = File::create(output_path)?;
            img.write_to_png(BufWriter::new(output_file))?;
          }
        }
        DynamicFvpHzc::Gray(hzc) => {
          for (i, img) in hzc.entries().iter().enumerate() {
            let output_path = args.output.join(format!("{filename}-{i}.png"));
            let output_file = File::create(output_path)?;
            img.write_to_png(BufWriter::new(output_file))?;
          }
        }
      }

      Ok(())
    })
    .collect::<Result<()>>()?;

  Ok(())
}
