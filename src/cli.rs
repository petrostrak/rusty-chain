use std::process::exit;

use clap::{Arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("rusty-chain")
            .version("0.1")
            .author("pit.trak@gmail.com")
            .about("blockchain in Rust: a simple blockchain")
            .subcommand(Command::new("print-chain").about("print all the chain blocks"))
            .subcommand(
                Command::new("get-balance")
                    .about("get balance in the blockchain")
                    .arg(Arg::new("ADDRESS")),
            )
            .subcommand(
                Command::new("create")
                    .about("create new blockchain")
                    .arg(Arg::new("ADDRESS")),
            )
            .subcommand(
                Command::new("send")
                    .about("send in the blockchain")
                    .arg(Arg::new("FROM"))
                    .arg(Arg::new("TO"))
                    .arg(Arg::new("AMOUNT")),
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone());
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("get-balance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(&address);
                let mut balance = 0;
                for out in utxos {
                    balance += out.value;
                }
                println!("Balance of '{}'; {}", address, balance);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            } else {
                println!("'FROM' not supplied");
                exit(1);
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("'TO' not supplied");
                exit(1);
            };

            let amount = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            } else {
                println!("'AMOUNT' not supplied");
                exit(1);
            };

            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            bc.add_block(vec![tx]);
            println!("success!");
        }

        if let Some(_) = matches.subcommand_matches("print-chain") {
            cmd_print_chain()?;
        }

        Ok(())
    }
}

fn cmd_print_chain() -> Result<()> {
    let bc = Blockchain::new()?;
    for b in bc.iter() {
        println!("{:#?}", b);
    }
    Ok(())
}
