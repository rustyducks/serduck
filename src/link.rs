// use anyhow::Result;


// pub trait Link {
//     fn send_msg(&self, t: LinkMessage) -> Result<()>;
// }

#[derive(Clone)]
pub struct LinkMessage {
    buffer: Vec<u8>
}

impl LinkMessage {
    pub fn from_bytes(buf: &[u8]) -> Self {
        let buffer = buf.iter().map(|c| *c).collect();
        LinkMessage { buffer}        
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..]
    }    
}
