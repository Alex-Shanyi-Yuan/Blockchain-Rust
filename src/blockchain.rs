use std::collections::HashMap;
use std::ops::RangeBounds;

use log::info;

use crate::block::Block;
use crate::errors::Result;
use crate::transaction::{TXOutput, Transaction};

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
        info!("Open Blockchain");

        let db = sled::open("data/blocks")?;
        let hash = db
            .get("LAST")?
            .expect("Must create a new block databse first");
        info!("Found block database");
        let last_hash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            current_hash: last_hash.clone(),
            db,
        })
    }

    // CreateBlockchain creates a new blockchain DB
    pub fn creat_blockchain(address: String) -> Result<Blockchain> {
        info!("Createing new Blockchain");

        let db = sled::open("data/blocks")?;
        info!("Createing new block database");
        let cbtx = Transaction::new_coinbase(address, String::from("GENSIS_COINBASE_DATA"))?;
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            current_hash: genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }

    // FindUnsepentTransactions returens a list of transactions containing unspent outputs
    // transactions whos outputs are not referenced by other inputs
    fn find_unspent_transactions(&self, address: &str) -> Vec<Transaction> {
        let mut spent_TXOs = HashMap::<String, Vec<i32>>::new();
        let mut unspent_TXs = Vec::new();

        for block in self.iter() {
            for tx in block.get_transactions() {
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.vout[index].can_be_unlock_with(address) {
                        unspent_TXs.push(tx.to_owned())
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.can_unlock_output_with(address) {
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }

        unspent_TXs
    }

    // FindUTXO finds and returns all unspent transaction outputs
    pub fn find_UTXO(&self, address: &str) -> Vec<TXOutput> {
        let mut utxos = Vec::<TXOutput>::new();
        let unspent_TXs = self.find_unspent_transactions(address);
        for tx in unspent_TXs {
            for out in &tx.vout {
                if out.can_be_unlock_with(&address) {
                    utxos.push(out.clone());
                }
            }
        }
        utxos
    }

    // FindUnspentTransactions returens a list of transactions containing unspent outputs
    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs = HashMap::<String, Vec<i32>>::new();
        let mut accmulated = 0;
        let unspent_TXs = self.find_unspent_transactions(address);

        for tx in unspent_TXs {
            for index in 0..tx.vout.len() {
                if tx.vout[index].can_be_unlock_with(address) && accmulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }
                    accmulated += tx.vout[index].value;

                    if accmulated >= amount {
                        return (accmulated, unspent_outputs);
                    }
                }
            }
        }
        (accmulated, unspent_outputs)
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<()> {
        let last_hash = String::from_utf8(self.db.get("LAST")?.unwrap().to_vec())?;
        let new_block = Block::new_block(transactions, last_hash, TARGET_HEXT).unwrap();
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
        // b.add_block("data".to_string());
        // b.add_block("data2".to_string());
        // b.add_block("data3".to_string());

        for item in b.iter() {
            println!("item {:?}", item)
        }
    }
}
