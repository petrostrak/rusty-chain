use crate::cli::Cli;
use crate::errors::Result;

mod block;

mod blockchain;
mod cli;
mod errors;
mod server;
mod transaction;
mod tx;
mod utxoset;
mod wallets;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
