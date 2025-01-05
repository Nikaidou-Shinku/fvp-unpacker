mod cli;
mod commands;
mod utils;

use clap::Parser;

use cli::Cli;

fn main() {
  let cli = Cli::parse();

  if let Err(error) = commands::run(&cli) {
    eprintln!("Error: {error}");
  }
}
