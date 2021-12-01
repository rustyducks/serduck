
use std::{io, thread, time::Duration};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::transport::Transport;
use crate::link::{LinkMessage};
use std::net::{SocketAddr, UdpSocket};

pub fn run(socket: UdpSocket, rx_msg: Receiver<LinkMessage>, rx_cmd: Receiver<usize>, sink: Sender<LinkMessage>) {
    let mut trans = Transport::new();

    let mut clients: Vec<SocketAddr> = vec![];

    loop {
        match rx_cmd.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        match rx_msg.try_recv() {
            Ok(msg) => {
                let buf = Transport::encode(&msg);
                for c in &clients {
                    let _ = socket.send_to(&buf, c);
                }
            },
            _ => {}
        }

        let mut buffer: [u8; 50] = [0; 50];
        match socket.recv_from(&mut buffer){
            Ok((nb, addr)) => {
                if !clients.contains(&addr) {
                    clients.push(addr);
                    println!("new client: {:?}", addr);
                }

                if let Ok(msg) = trans.put(&buffer[0..nb]) {
                    sink.send(msg).expect("Coordinator is down.");
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => eprintln!("{:?}", e),
        };

        
        thread::sleep(Duration::from_micros(10));    
    }
}
