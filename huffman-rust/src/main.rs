use bitvec::vec::BitVec;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct FrequencyMap {
    data: HashMap<u8, i32>,
}

fn byte_to_char(byte: &u8) -> char {
    char::from_u32(*byte as u32).expect("All u8 values should be a valid char")
}

impl FrequencyMap {
    fn new(data: HashMap<u8, i32>) -> Self {
        Self { data }
    }
}

impl From<&str> for FrequencyMap {
    fn from(data: &str) -> Self {
        let mut frequencies = HashMap::<u8, i32>::new();

        for data in data.bytes() {
            // increment map[char] by 1
            *frequencies.entry(data).or_insert(0) += 1;
        }

        FrequencyMap::new(frequencies)
    }
}

impl HuffmanNode {
    fn into_encoding_table(self, prefix: BitVec) -> HashMap<u8, BitVec> {
        let mut result = HashMap::new();

        if let Some(data) = self.data {
            // use a 1 bit if there is only one thing in the tree (no prefix yet)
            if prefix.is_empty() {
                result.insert(data, BitVec::from_slice(&[0]));
                return result;
            };
            result.insert(data, prefix.clone());
        };

        if let Some(left) = self.left {
            let mut prefix = prefix.clone();
            prefix.push(false);

            let left_result = left.into_encoding_table(prefix);

            result.extend(left_result);
        }

        if let Some(right) = self.right {
            let mut prefix = prefix.clone();
            prefix.push(true);

            let right_result = right.into_encoding_table(prefix);

            result.extend(right_result);
        }

        return result;
    }
}

#[derive(PartialEq, Eq)]
struct HuffmanNode {
    pub data: Option<u8>,
    pub frequency: i32,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(data: Option<u8>, frequency: i32) -> Self {
        Self {
            data,
            frequency,
            left: None,
            right: None,
        }
    }
}

impl HuffmanNode {
    fn debug(&self, indent: i32, level: i32) {
        println!(
            "{}{}",
            " ".repeat((indent * level) as usize),
            format!(
                "{:?} (freq: {})",
                // convert to char to print as chars
                self.data.as_ref().map(|v| char::from_u32(*v as u32)),
                self.frequency
            )
        );
        if let Some(left) = &self.left {
            left.debug(indent, level + 1)
        };
        if let Some(right) = &self.right {
            right.debug(indent, level + 1)
        };
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

fn main() {
    println!("Hello, world!");

    let frequency_map = FrequencyMap::from("mississippi");

    for (byte, count) in &frequency_map.data {
        println!("{:?} ({})", byte_to_char(byte), count)
    }

    let huffman_nodes = frequency_map
        .data
        .into_iter()
        .map(|(char, frequency)| Reverse(HuffmanNode::new(Some(char), frequency)))
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::from(huffman_nodes);

    while heap.len() > 1 {
        let Reverse(left) = heap.pop().unwrap();
        let Reverse(right) = heap.pop().unwrap();

        let merged = HuffmanNode {
            data: None,
            frequency: left.frequency + right.frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        heap.push(Reverse(merged));
    }

    let Reverse(huffman_tree) = heap.pop().unwrap();

    huffman_tree.debug(2, 0);

    let encoding_table = huffman_tree.into_encoding_table(Default::default());

    encoding_table.iter().for_each(|(byte, prefix)| {
        println!(
            "{} -> {}",
            byte_to_char(byte),
            prefix
                .iter()
                .map(|v| if v == true { "1" } else { "0" })
                .collect::<Vec<_>>()
                .concat()
        )
    });
}
