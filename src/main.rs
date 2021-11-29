//
//use std::cmp::PartialEq;
mod link;
mod transport;
mod serial_link;
mod udp_link;

use std::sync::mpsc;
use std::{thread, time::Duration};
use serial_link::{SerialLinkConfig};
use udp_link::UdpLink;

use link::Link;

use crate::link::Message;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};


const PORT1: &str = "/dev/ttyUSB0";
const UDP_SERVER: &str = "127.0.0.1:3456";

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let mut links: Vec<Box<dyn Link>> = Vec::new();
    let mut errors : Vec<String> = Vec::new();


    let (tx, rx) = mpsc::channel::<Message>();


    match UdpLink::new(UDP_SERVER, tx.clone(), 1) {
        Ok(ul) => links.push(Box::new(ul)),
        Err(e) => {
            let msg = format!("Error for {}: {:?}", PORT1, e);
            println!("{}", msg);
            errors.push(msg);
        },
    }

    let slc = SerialLinkConfig {
        port:PORT1.into(),
        baudrate: 57600,
        timeout: 1,
    };

    match slc.start(tx.clone()) {
        Ok(sl) => links.push(Box::new(sl)),
        Err(e) => {
            let msg = format!("Error for {}: {:?}", PORT1, e);
            println!("{}", msg);
            errors.push(msg);
        },
    }

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {

        if let Ok(msg) =  rx.try_recv() {
            //println!("yes!");
            for s in &links {
                s.send_msg(msg.clone())?;
            }
        }

        thread::sleep(Duration::from_millis(1));
    }

    println!("stopping..........");

    Ok(())
}

