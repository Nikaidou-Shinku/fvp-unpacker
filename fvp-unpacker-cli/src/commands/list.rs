use std::{fs::File, path::PathBuf};

use anyhow::Result;
use clap::Args;
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};
use fvp_unpacker_core::prelude::*;
use memmap2::Mmap;

use crate::utils::human_readable_size;

#[derive(Args)]
pub struct ListArgs {
  /// Input file path
  #[arg(short, long)]
  input: PathBuf,

  /// Print sizes like 1KiB, 234MiB, 2GiB, etc.
  #[arg(long)]
  human: bool,
}

pub fn list(args: &ListArgs) -> Result<()> {
  let input_file = File::open(&args.input)?;
  // SAFETY: it's not my fault :(
  let content = unsafe { Mmap::map(&input_file) }?;

  // TODO: handle other formats
  let arc = FvpBin::parse(content)?;

  let mut table = Table::new();
  table
    .load_preset(UTF8_FULL_CONDENSED)
    .set_header(["Filename", "Size"]);

  for entry in arc.entries() {
    let size = entry.data().len();

    table.add_row([
      entry.filename().to_string(),
      if args.human {
        human_readable_size(size)
      } else {
        size.to_string()
      },
    ]);
  }

  println!("{table}");

  Ok(())
}
