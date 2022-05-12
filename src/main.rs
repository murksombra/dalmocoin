use dalmocoinlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff; //difficulty, as we add more zeros, more difficult to mine
    let mut genesis = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    destination: "Dalmo".to_owned(),
                    value: 300,
                },
                transaction::Output {
                    destination: "Sara".to_owned(),
                    value: 350,
                },
            ],
        }],
        difficulty,
    );
    genesis.mine();
    println!("Mined genesis block {:?}", &genesis);

    let mut last_hash = genesis.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![genesis],
    };
    for i in 1..=10 {
        let mut block = Block::new(
            i,
            now(),
            last_hash,
            0,
            "Another block".to_owned(),
            difficulty,
        );
        block.mine();
        println!("Mined block {:?}", &block);

        last_hash = block.hash.clone();
        blockchain.blocks.push(block);

        // println!("Verify: {}", &blockchain.update(block)());
    }
}
