mod cli;

use std::{fs::{self, File}, io::BufWriter};

use clap::Parser;
use fvp_unpacker_core::{
  archive::{bin::FvpBin, hzc::FvpHzc},
  error::FvpResult,
  utils::fread::FvpBuffer,
};
use memmap2::Mmap;
use rayon::prelude::*;

use cli::Cli;

fn main() {
  if let Err(error) = run() {
    eprintln!("Error: {error}");
  }
}

fn run() -> FvpResult<()> {
  let cli = Cli::parse();

  let input_file = File::open(cli.input)?;
  let content = unsafe { Mmap::map(&input_file) }?;
  let arc: FvpBin = content.fread(0)?;

  if !cli.output.is_dir() {
    fs::create_dir_all(&cli.output)?;
  }

  arc
    .entries()
    .par_iter()
    .map(|entry| {
      let filename = entry.filename();
      let hzc: FvpHzc = entry.data().fread(0)?;

      for (i, img) in hzc.entries().enumerate() {
        let output_path = cli.output.join(format!("{filename}-{i}.png"));
        let output_file = File::create(output_path)?;
        img.write_to_png(BufWriter::new(output_file))?;
      }

      Ok(())
    })
    .collect::<FvpResult<()>>()?;

  Ok(())
}
