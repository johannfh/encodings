use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use bitvec::vec::BitVec;

struct FrequencyMap {
    data: HashMap<u8, i32>,
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
    fn into_encoding_table<'a>(self, prefix: Option<&BitVec>) -> HashMap<u8, BitVec> {
        let prefix: &BitVec = prefix.unwrap_or(&BitVec::new());

        let mut result = HashMap::new();

        if let Some(data) = self.data {
            result.insert(data, prefix.clone());
        };

        let left_result = if let Some(left) = self.left {
            let mut prefix = prefix.clone();
            prefix.push(false);
            left.into_encoding_table(Some(&prefix))
        } else {
            todo!()
        };

        return result;
    }

    fn 
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
        println!(
            "{:?} ({})",
            char::from_u32(*byte as u32).expect("All u8 values should be a valid char"),
            count
        )
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
}
