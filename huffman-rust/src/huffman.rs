use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use bitvec::prelude::*;

fn get_frequencies(data: &[u8]) -> HashMap<u8, u8> {
    let mut result = HashMap::<u8, u8>::new();

    for value in data {
        if let Some(frequency) = result.get_mut(value) {
            *frequency += 1;
        } else {
            result.insert(*value, 1);
        }
    }

    return result;
}

#[derive(Default, Hash, PartialEq, Eq, Debug)]
pub struct HuffmanNode {
    pub data: Option<u8>,
    frequency: u8,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl From<&[u8]> for HuffmanNode {
    fn from(value: &[u8]) -> Self {
        let frequencies = get_frequencies(value);

        let mut heap: BinaryHeap<Reverse<HuffmanNode>> = frequencies
            .into_iter()
            .map(|(data, frequency)| HuffmanNode::new(data, frequency))
            // use Reverse<T> to convert max-heap to min-heap
            .map(|v| Reverse(v))
            .collect();

        while heap.len() > 1 {
            let Reverse(left) = heap
                .pop()
                .expect("There should be at least two elements left in the heap.");

            let Reverse(right) = heap
                .pop()
                .expect("There should be at least one element left in the heap.");

            let merged = HuffmanNode {
                data: None,
                frequency: left.frequency + right.frequency,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            heap.push(Reverse(merged));
        }

        heap.pop().map(|v| v.0).unwrap_or(Default::default())
    }
}

impl HuffmanNode {
    pub fn new(data: u8, frequency: u8) -> Self {
        Self {
            data: Some(data),
            frequency,
            left: None,
            right: None,
        }
    }

    pub fn debug(&self, indent: i32, level: i32) {
        let indentation = " ".repeat((indent * level) as usize);

        let info = if let Some(data) = &self.data {
            if let Ok(utf8) = String::from_utf8(Vec::from(&[*data])) {
                format!("{:?} ({})", utf8, self.frequency)
            } else {
                format!("{:?} ({})", data, self.frequency)
            }
        } else {
            format!("({})", self.frequency)
        };

        println!("{}{}", indentation, info);

        if let Some(left) = &self.left {
            left.debug(indent, level + 1);
        };

        if let Some(right) = &self.right {
            right.debug(indent, level + 1);
        };
    }

    pub fn get_encoding_map(&self, prefix: BitVec) -> HashMap<u8, BitVec> {
        let mut result = HashMap::new();

        if let Some(data) = self.data {
            result.insert(data, prefix.clone());
        }

        if let Some(left) = &self.left {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            let left_map = left.get_encoding_map(left_prefix);
            result.extend(left_map);
        }

        if let Some(right) = &self.right {
            let mut right_prefix = prefix.clone();
            right_prefix.push(true);
            let right_map = right.get_encoding_map(right_prefix);
            result.extend(right_map);
        }

        return result;
    }

    pub fn encode_data(&self, data: &[u8]) -> BitVec {
        let encoding_map = self.get_encoding_map(BitVec::new());

        data.iter()
            .filter_map(|value| encoding_map.get(value))
            .fold(BitVec::new(), |a, b| {
                let mut joined = BitVec::new();
                joined.extend(a);
                joined.extend(b);
                return joined;
            })
    }
}
