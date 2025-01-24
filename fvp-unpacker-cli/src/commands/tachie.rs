use std::{
  collections::HashMap,
  fs::{self, File},
  io::BufWriter,
  path::PathBuf,
};

use anyhow::{Result, anyhow, bail};
use clap::Args;
use fvp_unpacker_core::prelude::*;
use memmap2::Mmap;
use rayon::prelude::*;

#[derive(Args)]
pub struct TachieArgs {
  /// Input file path
  #[arg(short, long)]
  input: PathBuf,

  /// Output directory path
  #[arg(short, long, default_value = "./output")]
  output: PathBuf,

  /// The filename containing the character's tachie(立ち絵), e.g. `CHR_雪々_喜_着物U`
  #[arg(short, long)]
  character: String,
}

pub fn tachie(args: &TachieArgs) -> Result<()> {
  if !args.output.is_dir() {
    fs::create_dir_all(&args.output)?;
  }

  let input_file = File::open(&args.input)?;
  // SAFETY: it's not my fault :(
  let content = unsafe { Mmap::map(&input_file) }?;

  // TODO: handle other formats
  let arc = FvpBin::parse(content)?;

  let entries: HashMap<_, _> = arc
    .entries()
    .iter()
    .map(|entry| (entry.filename(), entry))
    .collect();

  // TODO: multiple characters
  let base = *entries
    .get(args.character.as_str())
    .ok_or(anyhow!("No such character"))?;
  let facial_expression = *entries
    .get(format!("{}_表情", args.character).as_str())
    .ok_or(anyhow!("Can not find facial expression"))?;

  drop(entries);

  let DynamicFvpHzc::Bgra(base_hzc) = DynamicFvpHzc::parse(base.data())? else {
    bail!("The tachie must be BGRA images");
  };

  let base_image = {
    let base_entries = base_hzc.entries();

    if base_entries.len() != 1 {
      bail!("The count of images of the tachie must be 1");
    }

    &base_entries[0]
  };

  let DynamicFvpHzc::Bgra(facial_expression_hzc) = DynamicFvpHzc::parse(facial_expression.data())?
  else {
    bail!("The facial expression must be BGRA images");
  };

  facial_expression_hzc
    .entries()
    .par_iter()
    .enumerate()
    .map(|(i, facial_expression)| {
      let mut base = base_image.clone();

      base
        .data
        .sub_image_mut(
          facial_expression.offset.0.into(),
          facial_expression.offset.1.into(),
          facial_expression.data.width(),
          facial_expression.data.height(),
        )
        .rows_mut()
        .zip(facial_expression.data.rows())
        .for_each(|(dst, src)| dst.copy_from_slice(src));

      let output_path = args.output.join(format!("{}-{i}.png", args.character));
      let output_file = File::create(output_path)?;
      base.write_to_png(BufWriter::new(output_file))?;

      Ok(())
    })
    .collect::<Result<()>>()?;

  Ok(())
}
