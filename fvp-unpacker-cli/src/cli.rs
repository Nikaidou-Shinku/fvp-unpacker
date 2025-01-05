use clap::Parser;

use crate::commands::{ListArgs, UnpackArgs};

#[derive(Parser)]
#[command(version, author, about = "A blazing fast tool to unpack FVP archive", long_about = None)]
pub enum Cli {
  /// Unpack all files from the archive without additional processing
  Unpack(UnpackArgs),

  /// List files that can be unpacked
  List(ListArgs),
}
