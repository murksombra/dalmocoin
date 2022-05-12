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
    let mut blockchain = Blockchain::new();
    blockchain
        .update(genesis)
        .expect("Failed like there's no tomorrow");
}
