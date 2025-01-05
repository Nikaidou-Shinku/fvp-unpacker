use std::{
  fs::{self, File},
  io::BufWriter,
};

use anyhow::Result;
use fvp_unpacker_core::prelude::*;
use memmap2::Mmap;
use rayon::prelude::*;

use crate::cli::Cli;

pub fn unpack(args: &Cli) -> Result<()> {
  if !args.output.is_dir() {
    fs::create_dir_all(&args.output)?;
  }

  let input_file = File::open(&args.input)?;
  // SAFETY: it's not my fault :(
  let content = unsafe { Mmap::map(&input_file) }?;

  let arc: FvpBin = content.fread(0)?;

  arc
    .entries()
    .par_iter()
    .map(|entry| {
      let filename = entry.filename();
      // TODO: handle other file formats
      let hzc: FvpHzc = entry.data().fread(0)?;

      for (i, img) in hzc.entries().enumerate() {
        let output_path = args.output.join(format!("{filename}-{i}.png"));
        let output_file = File::create(output_path)?;
        img.write_to_png(BufWriter::new(output_file))?;
      }

      Ok(())
    })
    .collect::<Result<()>>()?;

  Ok(())
}
