use anyhow::Result;


pub trait Link {
    fn send_msg(&self, t: Message) -> Result<()>;
}

#[derive(Clone)]
pub struct Message {
    buffer: Vec<u8>
}

impl Message {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let buffer = buf.iter().map(|c| *c).collect();
        Message { buffer}        
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..]
    }    
}
