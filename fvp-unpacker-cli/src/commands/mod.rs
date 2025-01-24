mod list;
mod tachie;
mod unpack;

use anyhow::Result;

use crate::cli::Cli;
pub use list::ListArgs;
pub use tachie::TachieArgs;
pub use unpack::UnpackArgs;

pub fn run(args: &Cli) -> Result<()> {
  match args {
    Cli::Unpack(args) => unpack::unpack(args),
    Cli::List(args) => list::list(args),
    Cli::Tachie(args) => tachie::tachie(args),
  }
}
