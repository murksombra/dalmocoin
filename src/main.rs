use dalmocoinlib::*;

fn main() {
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis Block".to_owned());
    let hash = block.hash();
    block.hash = hash;
    println!("{:?}", &block);
}
