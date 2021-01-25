use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        88765,
        "Genesis block".to_owned(),
        0x000ffffffffffffffffffffffffffff,
    );

    block.hash = block.hash();

    println!("{:?}", &block);

    // block.mine();

    // println!("{:?}", &block);
}
