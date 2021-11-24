//
//use std::cmp::PartialEq;
mod link;
mod transport;
mod serial_link;
mod udp_link;

use std::sync::mpsc;
use std::{thread, time::Duration};
use serial_link::{SerialLinkConfig, SerialLink};
use udp_link::UdpLink;

use link::Link;

const PORT1: &str = "/dev/ttyUSB0";
const UDP_SERVER: &str = "127.0.0.1:3456";

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {


    

    let mut serials: Vec<Box<SerialLink>> = Vec::new();
    let mut udps: Vec<UdpLink> = Vec::new();
    let mut errors : Vec<String> = Vec::new();


    let (tx, _rx) = mpsc::channel::<usize>();


    match UdpLink::new(UDP_SERVER, tx.clone()) {
        Ok(ul) => udps.push(ul),
        Err(e) => {
            let msg = format!("Error for {}: {:?}", PORT1, e);
            println!("{}", msg);
            errors.push(msg);
        },
    }

    // let slc = SerialLinkConfig {
    //     port:PORT1.into(),
    //     baudrate: 38400,
    //     timeout: 1,
    // };

    // match slc.start(tx.clone()) {
    //     Ok(sl) => serials.push(Box::new(sl)),
    //     Err(e) => {
    //         let msg = format!("Error for {}: {:?}", PORT1, e);
    //         println!("{}", msg);
    //         errors.push(msg);
    //     },
    // }



    for _ in 0..10000 {

        if let Ok(msg) =  _rx.try_recv() {
            println!("msg: {}", msg);
            for s in &serials {
                s.send_msg(456)?;
            }
            for s in &udps {
                s.send_msg(456)?;
            }
        }

        thread::sleep(Duration::from_millis(1));    
    }


    for s in serials {
        s.stop();
    }
    for s in udps {
        s.stop();
    }





    

    //loop
    //{
        // // Receives a single datagram message on the socket. If `buf` is too small to hold
        // // the message, it will be cut off.
        // let mut buf = [0; 10];
        // let (amt, src) = socket.recv_from(&mut buf)?;
        
        // if !clients.iter().any(|sa| sa == &Client::UDP(src)) {
        //     clients.push(Client::UDP(src));
        // }


        // // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        // let buf = &mut buf[..amt];
        // buf.reverse();

        // let _ = clients.iter().map(|c| match c {
        //     Client::UDP(addr) => socket.send_to(buf, &addr),
        //     _ => panic!(""),
            
        // }).collect::<Vec<_>>();

        //socket.send_to(buf, &src)?;
    //} // the socket is closed here
    Ok(())
}

