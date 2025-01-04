use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, author, about = "A blazing fast tool to unpack FVP archive", long_about = None)]
pub struct Cli {
  /// Input file path
  #[arg(short, long)]
  pub input: PathBuf,

  /// Output directory
  #[arg(short, long, default_value = "./output")]
  pub output: PathBuf,
}
