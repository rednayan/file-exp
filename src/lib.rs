pub mod huffman_tree;
use anyhow::Result;
use bit_vec::BitVec;
use std::collections::HashMap;

#[derive(Debug, Copy, Ord, Clone, Eq, PartialEq, PartialOrd)]
pub struct PQItem {
    pub value: u8,
    pub priority: u8,
}

impl PQItem {
    pub fn new(value: u8, priority: u8) -> Self {
        Self { value, priority }
    }
}

#[derive(Debug)]
pub struct PQArray {
    pub p_arr: Vec<PQItem>,
}

impl PQArray {
    pub fn new(pq_item: PQItem) -> Self {
        Self {
            p_arr: vec![pq_item],
        }
    }

    pub fn enqueue(&mut self, pq_item: PQItem) {
        &self.p_arr.push(pq_item);
        &self.p_arr.sort_by(|x, y| y.value.cmp(&x.value));
    }

    pub fn peek(&self) -> &PQItem {
        let min_prio = self
            .p_arr
            .iter()
            .min_by(|x, y| x.priority.cmp(&y.priority))
            .unwrap();
        min_prio
    }

    pub fn dequeue(&mut self) -> Option<PQItem> {
        // &self.p_arr.sort_by(|x, y| x.priority.cmp(&y.priority));
        let item = &self.p_arr.pop();
        item.clone()
    }
}

pub fn compress(text_bytes: &Vec<u8>) -> Result<Vec<BitVec>> {
    //do the compression here
    let mut occur_map: HashMap<u8, u8> = HashMap::new();

    for i in text_bytes {
        match occur_map.get(i) {
            Some(count) => {
                occur_map.insert(i.to_owned(), count + 1);
            }
            None => {
                occur_map.insert(i.to_owned(), 1);
            }
        }
    }
    let tree = huffman_tree::huffman_tree(&occur_map);
    let encode = tree.to_encoder();
    println!("{:?}", tree);

    // let mut pq_array: PQArray = PQArray::new(PQItem {
    //     value: 0,
    //     priority: 0,
    // });
    // pq_array.dequeue();

    // for (k, v) in occur_map {
    //     let pq_item: PQItem = PQItem::new(v, k);
    //     pq_array.enqueue(pq_item);
    // }

    // huffman_tree::huffman_tree(&mut pq_array);

    // let min = pq_array.peek();
    // pq_array.dequeue();
    // for i in pq_array.p_arr {
    //     println!("{:?}:{:?}", i.value, i.priority);
    // }
    let mut compressed_vec: Vec<BitVec> = Vec::new();

    for i in text_bytes {
        for (v, l) in &encode {
            if i == v {
                compressed_vec.push(l.clone());
            }
        }
    }
    Ok(compressed_vec)
}
