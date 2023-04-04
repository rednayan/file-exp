use bit_vec::BitVec;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use Tree::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    Leaf {
        freq: u8,
        text_byte: u8,
    },
    Node {
        freq: u8,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    pub fn freq(&self) -> u8 {
        match self {
            Leaf { freq, .. } => *freq,
            Node { freq, .. } => *freq,
        }
    }

    pub fn text_bytes(&self) -> Option<u8> {
        match self {
            Leaf { text_byte, .. } => Some(text_byte.clone()),
            Node { .. } => None,
        }
    }

    pub fn left(&self) -> Option<&Tree> {
        match self {
            Node { left, .. } => Some(left),
            Leaf { .. } => None,
        }
    }
    pub fn rigth(&self) -> Option<&Tree> {
        match self {
            Node { right, .. } => Some(right),
            Leaf { .. } => None,
        }
    }

    pub fn to_encoder(&self) -> HashMap<u8, BitVec> {
        let mut encoder = HashMap::new();

        let mut stack = vec![(self, BitVec::new())];
        while !stack.is_empty() {
            let (node, path) = stack.pop().unwrap();
            match node {
                Leaf { text_byte, .. } => {
                    encoder.insert(text_byte.clone(), path.clone());
                }
                Node { left, right, .. } => {
                    let mut left_path = path.clone();
                    left_path.push(false);
                    stack.push((left, left_path));

                    let mut right_path = path.clone();
                    right_path.push(true);
                    stack.push((right, right_path));
                }
            }
        }
        encoder
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn huffman_tree(freqs: &HashMap<u8, u8>) -> Tree {
    let mut heap = BinaryHeap::new();
    for (text_byte, freq) in freqs {
        let (text_byte, freq) = (text_byte.clone(), *freq);
        heap.push(Reverse(Leaf { freq, text_byte }))
    }

    while heap.len() > 1 {
        let node1 = heap.pop().unwrap().0;
        let node2 = heap.pop().unwrap().0;

        let merged_node = Node {
            freq: node1.freq() + node2.freq(),
            left: Box::new(node1),
            right: Box::new(node2),
        };
        heap.push(Reverse(merged_node))
    }
    println!("{:?}", &heap);
    heap.pop().unwrap().0
}
