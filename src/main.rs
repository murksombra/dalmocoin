use dalmocoinlib::*;

fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        68496,
        "Genesis Block".to_owned(),
        0x0000ffffffffffffffffffffffffffff,
    );
    block.hash = block.hash();
    println!("{:?}", &block);
    // block.mine();
    // println!("{:?}", &block);
}
