use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("Blockchain-Rust-App")
            .version("0.1.0")
            .author("alexyuan150298224@gmail.com")
            .about("Rust Blockchain sample app")
            .subcommand(Command::new("printchain").about("print all the blocks in blockchain"))
            .subcommand(
                Command::new("addblock")
                    .about("insert a block in to blockchain")
                    .arg(arg!(<DATA> " 'The Block Data ' ")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.bc.add_block(String::from(c))?;
            } else {
                println!("Missing input arg: DATA");
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            self.print_chain();
        }

        Ok(())
    }

    pub fn print_chain(&self) {
        for b in &mut self.bc.iter() {
            println!("block {:#?}", b);
        }
    }
}
