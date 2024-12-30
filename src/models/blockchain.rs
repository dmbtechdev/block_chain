use super::block::Block;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs::OpenOptions,io::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    difficulty: usize, // Number of leading zeros required in block hash.
}

impl Blockchain {

    // Create a new blockchain
    pub fn new(difficulty: usize) -> Self {
        
        // Create genesis block (the first block) in the chain as an empty block
        let mut genesis_block = Block::new(  
            0,
            vec![],
            String::default()
        );

        let genesis_block_difficulty = 4;

        // Mine the genesis block with  difficulty
        genesis_block.mine(genesis_block_difficulty, 0);
        
        // Create chain starting from the genesis chain.
        let chain = vec![genesis_block.clone()];

        Self {
            chain,
            difficulty,
        }
    }

    // Add a block to the chain
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    // Write the blockchain to a file
    pub fn write_to_file(&self) -> Result<()> {
        let file_path = "w_block_chain.json";
        let file = OpenOptions::new().write(true).create(true).open(file_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

}
