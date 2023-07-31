use crate::block::Block;
use crate::errors::Result;

const TARGET_HEXT: usize = 4;

// Allows using println!("{:?}", some_block) for debugging
#[derive(Debug, Clone)]
// acts as reversed linkedlist
pub struct Blockchain {
    pub current_hash: String,
    pub db: sled::Db,
}
pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        let db = sled::open("data/blocks")?;
        match db.get("LAST")? {
            Some(hash) => {
                let last_hash = String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    current_hash: last_hash,
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
        let last_hash = String::from_utf8(self.db.get("LAST")?.unwrap().to_vec())?;
        let new_block = Block::new_block(data, last_hash, TARGET_HEXT).unwrap();
        self.db
            .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.bc.db.get(&self.current_hash) {
            return match encoded_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut b = Blockchain::new().unwrap();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());

        for item in b.iter() {
            println!("item {:?}", item)
        }
    }
}
