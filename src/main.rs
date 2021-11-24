//use std::net::{SocketAddr, UdpSocket};
//use std::cmp::PartialEq;
mod link;
mod transport;
use core::panic;
use std::fmt::format;
use std::sync::mpsc;
use std::{thread, time::Duration};

use link::SerialLink;
//use link::{SerialLink};


// enum Client {
//     UDP(SocketAddr),
//     SERIAL,
// }

// impl PartialEq for Client {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::UDP(l0), Self::UDP(r0)) => l0 == r0,
//             _ => false,
//         }
//     }
// }

const PORT1: &str = "/dev/ttyUSB0";


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {


    //let (tx, rx) = mpsc::channel::<bool>();






    //let mut clients: Vec<Client> = Vec::new();
    //let socket = UdpSocket::bind("127.0.0.1:34254")?;

    let mut serials: Vec<SerialLink> = Vec::new();
    let mut errors : Vec<String> = Vec::new();


    let (tx, _rx) = mpsc::channel::<usize>();

    let slc = link::SerialLinkConfig {
        port:PORT1.into(),
        baudrate: 38400,
        timeout: 1,
    };

    if let Ok(sl) = slc.start(tx.clone()) {
        serials.push(sl);
    } else {
        errors.push(format!("{}", PORT1));
        panic!("merde");
    }



    for _ in 0..10000 {

        if let Ok(msg) =  _rx.try_recv() {
            println!("msg: {}", msg);
        }

        thread::sleep(Duration::from_millis(1));    
    }

    for s in serials {
        s.send_msg(456)?;
        s.join();
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

