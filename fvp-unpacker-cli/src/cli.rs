use clap::Parser;

use crate::commands::{ListArgs, TachieArgs, UnpackArgs};

#[derive(Parser)]
#[command(version, author, about = "A blazing fast tool to unpack FVP archive", long_about = None)]
pub enum Cli {
  /// Unpack all files from the archive without additional processing
  Unpack(UnpackArgs),

  /// List files that can be unpacked
  List(ListArgs),

  /// Process the original image and output the tachie(立ち絵).
  Tachie(TachieArgs),
}
