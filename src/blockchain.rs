use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty verification failed");
                return false;
            } else if i != 0 {
                //no genesis block
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                }
            } else {
                //Genesis block
                if block.previous_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_invalid");
                    return false;
                }
            }
        }
        true
    }
}
