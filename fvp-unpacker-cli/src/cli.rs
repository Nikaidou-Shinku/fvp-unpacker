use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, author, about = "A blazing fast tool to unpack FVP archive", long_about = None)]
pub struct Cli {
  /// Input file path
  #[arg(short, long)]
  pub input: PathBuf,

  /// Output directory path
  #[arg(short, long, default_value = "./output")]
  pub output: PathBuf,

  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Unpack all files from the archive without additional processing
  Unpack,
}
