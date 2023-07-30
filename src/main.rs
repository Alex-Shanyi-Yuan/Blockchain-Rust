mod block;
mod blockchain;
mod cli;
mod errors;

use crate::cli::Cli;
use crate::errors::Result;

fn main() -> Result<()> {
    println!("Welcome to Blockchain CLI");

    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
