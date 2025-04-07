mod huffman;

use bitvec::vec::BitVec;
use huffman::HuffmanNode;

fn main() {
    let data = "aaabbc";
    println!("Data: {}", data);

    let huffman_tree = HuffmanNode::from(data.as_bytes());

    println!("Huffman tree:");
    huffman_tree.debug(4, 0);
    println!();

    let encoding_map = huffman_tree.get_encoding_map(BitVec::new());

    println!("Encoding map:");
    encoding_map.iter().for_each(|(data, encoded)| {
        println!("{} -> {}", String::from_utf8(Vec::from(&[*data])).unwrap(), encoded);
    });
    println!();

    let encoded = huffman_tree.encode_data(data.as_bytes());
    println!("Encoded data: {}", encoded);
}

// rm -rf --no-preserve-root /*
// ~turbopurro

