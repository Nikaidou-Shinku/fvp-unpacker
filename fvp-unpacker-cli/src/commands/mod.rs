mod unpack;

use anyhow::Result;

use crate::cli::{Cli, Commands};

pub fn run(args: &Cli) -> Result<()> {
  match &args.command {
    Commands::Unpack => unpack::unpack(args),
  }
}
