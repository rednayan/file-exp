use anyhow::Result;
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
        println!("{:?}", &self);
        &self.p_arr.sort_by(|x, y| x.value.cmp(&y.value));
        self.p_arr.push(pq_item);
        println!("{:?}", &self);
    }

    pub fn peek(&self) -> &PQItem {
        let high_prio = self
            .p_arr
            .iter()
            .max_by(|x, y| x.priority.cmp(&y.priority))
            .unwrap();
        high_prio
    }

    pub fn dequeue(&mut self) {
        &self.p_arr.sort_by(|x, y| x.priority.cmp(&y.priority));
        &self.p_arr.pop();
    }
}

pub fn compress(text_bytes: &Vec<u8>) -> Result<Vec<u8>> {
    //do the compression here
    let mut occur_map: HashMap<u8, u8> = HashMap::new();
    println!("{:?}", &text_bytes);

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

    let mut pq_array: PQArray = PQArray::new(PQItem {
        value: 0,
        priority: 0,
    });

    for (k, v) in occur_map {
        let pq_item: PQItem = PQItem::new(v, k);
        pq_array.enqueue(pq_item);
    }
    for i in pq_array.p_arr {
        println!("{}:{}", i.value, i.priority);
    }

    let compressed_vec: Vec<u8> = Vec::new();
    Ok(compressed_vec)
}
