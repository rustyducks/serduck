use std::time::Duration;

use serialport::{SerialPort, Result};



pub trait Link {
    
}


pub struct SerialLink {
    port: String,
    baudrate: u32,
    buffer: &[u8],
    serial : Box<dyn SerialPort>
}


impl SerialLink {
    pub fn new(port: &str, baudrate: u32, timeout_ms:u64) -> Result<Self>{
        let serial = serialport::new(port, baudrate)
            .timeout(Duration::from_millis(timeout_ms))
            .open()?;
        Ok(SerialLink {port: port.to_string(), baudrate, serial})
    }

}

