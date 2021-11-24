use std::usize;



pub struct Transport {
    buffer: Vec<u8>,
}

impl Transport {

    pub fn new() -> Self {
        Transport{buffer: vec![]}
    }

    pub fn put(&mut self, buf_in: &[u8]) -> Result<usize, ()> {
        if buf_in.contains(&0xFF) {
            Ok(buf_in.len())
        } else {
            Err(())    
        }
    }
}
