//use core::slice::SlicePattern;
use std::usize;

use crate::link::Message;

enum RcvState {
    START1,
    START2,
    LEN,
    PAYLOAD(u8),    // nb bytes remaining to complete the message
    CHK,
}

pub struct Transport {
    state: RcvState,
    buffer: Vec<u8>,
}

impl Transport {

    pub fn new() -> Self {
        Transport{state:RcvState::START1, buffer: vec![]}
    }

    fn checksum(buffer: &[u8]) -> u8 {
        buffer.iter().fold(0, |acc, elt| acc ^ elt)
    }

    pub fn put(&mut self, buf_in: &[u8]) -> Result<Message, ()> {

        for c in buf_in {
            match self.state {
                RcvState::START1 => {
                    if *c == 0xFF {
                        self.state = RcvState::START2;
                    }
                },
                RcvState::START2 => {
                    if *c == 0xFF {
                        self.state = RcvState::LEN;
                    } else {
                        self.state = RcvState::START1;
                    }
                },
                RcvState::LEN => {
                    self.state = RcvState::PAYLOAD(*c);
                    self.buffer.clear();
                },
                RcvState::PAYLOAD(n) => {
                    self.buffer.push(*c);
                    let n = n-1;
                    if n > 0 {
                        self.state = RcvState::PAYLOAD(n);    
                    }
                    else {
                        self.state = RcvState::CHK;
                    }
                },
                RcvState::CHK => {
                    self.state = RcvState::START1;
                    return if Self::checksum(self.buffer.as_slice()) == *c {
                        let msg = Message::from_bytes(self.buffer.as_slice());
                        Ok(msg)
                    } else {
                        Err(())
                    }
                },
            }
        }

        Err(())
        



        // if buf_in.contains(&112) {
        //     let msg: Vec<u8> = buf_in.iter().rev().map(|c| *c).collect();
        //     let msg = Message::from_bytes(&msg[..]);
        //     Ok(msg)
        // } else {
        //     Err(())    
        // }
    }

    pub fn encode(msg: &Message) -> Vec<u8> {
        let payload = msg.as_bytes();
        let mut buf: Vec<u8> = vec![0xFF, 0xFF, payload.len() as u8];
        buf.extend_from_slice(payload);
        buf.push(Self::checksum(payload));
        buf
    }

}
