use clap::{Arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;

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
        let matches = Command::new("rusty-chain")
            .version("0.1")
            .author("pit.trak@gmail.com")
            .about("blockchain in Rust: a simple blockchain")
            .subcommand(Command::new("print-chain").about("print all the chain blocks"))
            .subcommand(
                Command::new("add-block")
                    .about("adds a block in the blockchain")
                    .arg(Arg::new("DATA")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("add-block") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.addblock(String::from(c))?;
            } else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) = matches.subcommand_matches("print-chain") {
            self.print_chain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(vec![])
    }

    fn print_chain(&mut self) {
        for b in &mut self.bc.iter() {
            println!("block: {:#?}", b);
        }
    }
}
