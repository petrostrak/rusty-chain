use cli::Cli;

mod block;
mod blockchain;
mod cli;
mod errors;
mod transaction;

use crate::errors::Result;

// cargo run print-chain
// cargo run add-block "value"

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}
