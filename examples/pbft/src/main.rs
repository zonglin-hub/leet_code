use sha2::{Digest, Sha256};

const DIFFICULTY: u32 = 5;

struct Block {
    nonce: u32,
    data: String,
}

impl Block {
    fn new(data: String) -> Block {
        Block { nonce: 0, data }
    }

    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", self.nonce, self.data));
        format!("{:x}", hasher.finalize())
    }

    fn mine(&mut self) {
        loop {
            if self.hash().starts_with(&"0".repeat(DIFFICULTY as usize)) {
                return;
            }
            self.nonce += 1;
        }
    }
}

fn main() {
    let data = String::from("hello world");
    let mut block = Block::new(data);
    block.mine();
    println!("nonce: {:?}", block.nonce);
    println!("Hash: {:?}", block.hash(),);
}
