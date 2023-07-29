use std::time::SystemTime

const TARGET_HEXS: usize = 4;

pub struct Block {
    timestamp: u128,        // Time when block is created
    transaction: String,    //
    pre_block_hash: String, //
    hash: String,           //
    height: usize,          //
    nonce: i32,             //
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    pub fn new(data: String, pre_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
        let mut block = Block {
            timestamp: timestamp,
            transaction: data,
            pre_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn prepare_hash_data($self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce,
        );
        let bytes:<Vec<u8>> = bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn validate($self) -> Result<bool> {
        let data: Vec<u8> = $self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
    }
}