use crate::block::Block;
use crate::errors::Result;
use sled::Db;

const TARGET_HEXT: usize = 4;

#[derive(Debug)]
pub struct Blockchain {
    current_hash: String,
    db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        let db: Db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) => {
                let lasthash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    current_hash: lasthash,
                    db,
                })
            }
            None => {
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = Blockchain {
                    current_hash: block.get_hash(),
                    db,
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let lasthash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data, String::from_utf8(lasthash.to_vec())?, TARGET_HEXT)?;
        self.db.insert(
            new_block.get_hash().as_bytes(),
            bincode::serialize(&new_block)?,
        )?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }
}
