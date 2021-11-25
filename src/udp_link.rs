
use std::thread::JoinHandle;
use std::{io, thread, time::Duration};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::transport::Transport;
use anyhow::Result;
use crate::link::{Link, Message};
use std::net::{SocketAddr, UdpSocket};


pub struct UdpLink {
    pub server_addr: String,
    tx_msg: Sender<Message>,
    tx_cmd: Sender<usize>,
    th: Option<JoinHandle<()>>
}




impl UdpLink {
    pub fn new(addr: &str, sink: Sender<Message>, timeout: u64) -> Result<UdpLink>{
        let socket = UdpSocket::bind(addr)?;
        socket.set_read_timeout(Some(Duration::from_millis(timeout))).expect("UDP set timeout failed");

        let (tx_msg, rx_msg) = mpsc::channel::<Message>();
        let (tx_cmd, rx_cmd) = mpsc::channel::<usize>();
        let th = thread::spawn(move || UdpLink::run(socket, rx_msg, rx_cmd, sink));
        
        Ok(Self{
            server_addr: addr.into(),
            tx_msg,
            tx_cmd,
            th: Some(th),
        })
        
    }

    fn run(socket: UdpSocket, rx_msg: Receiver<Message>, rx_cmd: Receiver<usize>, sink: Sender<Message>) {
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
                    for c in &clients {
                        let _ = socket.send_to(&Transport::encode(&msg), c);
                    }
                },
                _ => {}
            }

            let mut buffer: [u8; 50] = [0; 50];
            match socket.recv_from(&mut buffer){
                Ok((nb, addr)) => {
                    if !clients.contains(&addr) {
                        clients.push(addr);
                    } else {
                        println!("client known!!!!");
                    }
                    //println!("{}", nb);
                    if let Ok(msg) = trans.put(&buffer[0..nb]) {
                        sink.send(msg).expect("Coordinator is down.");
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
                Err(e) => eprintln!("{:?}", e),
            };

            
            thread::sleep(Duration::from_millis(1));    
        }
    }



    
}

impl Link for UdpLink {
    fn send_msg(&self, t: Message) -> Result<()> {
        self.tx_msg.send(t)?;
        Ok(())
    }
}


impl Drop for UdpLink {
    fn drop(&mut self) {
        self.tx_cmd.send(0).unwrap();
        self.th.take().unwrap().join().unwrap();
        println!("UdpLink destroyed");
    }
}
