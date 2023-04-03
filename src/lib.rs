use anyhow::Result;

#[derive(Debug, Copy, Clone)]
pub struct PQItem {
    pub value: u32,
    pub priority: u32,
}

#[derive(Debug)]
pub struct PQArray {
    pub p_arr: Vec<PQItem>,
}

impl PQItem {
    pub fn new(value: u32, priority: u32) -> Self {
        Self { value, priority }
    }
}

impl PQArray {
    pub fn new(pq_item: PQItem) -> Self {
        Self {
            p_arr: vec![pq_item],
        }
    }

    pub fn enqueue(&mut self, pq_item: PQItem) {
        self.p_arr.push(pq_item);
    }

    // pub fn peek(&self) -> PQItem {

    // }
}

pub fn create_prio_queue() {
    let pq_item: PQItem = PQItem::new(2, 3);
    let pq_item2: PQItem = PQItem::new(4, 5);
    let mut pq_array: PQArray = PQArray::new(pq_item);
    PQArray::enqueue(&mut pq_array, pq_item2);
    println!("{:?}", &pq_array);
}

pub fn compress(_text_bytes: &Vec<u8>) -> Result<Vec<u8>> {
    //do the compression here
    create_prio_queue();
    let compressed_vec: Vec<u8> = Vec::new();
    Ok(compressed_vec)
}
