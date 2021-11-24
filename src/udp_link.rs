
use std::thread::JoinHandle;
use std::{io, thread, time::Duration};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use crate::transport::Transport;
use anyhow::Result;
use crate::link::Link;
use std::net::{SocketAddr, UdpSocket};


pub struct UdpLink {
    pub server_addr: String,
    tx_msg: Sender<usize>,
    tx_cmd: Sender<usize>,
    th: JoinHandle<()>
}




impl UdpLink {
    pub fn new(addr: &str, sink: Sender<usize>) -> Result<UdpLink>{
        let socket = UdpSocket::bind(addr)?;


        let (tx_msg, rx_msg) = mpsc::channel::<usize>();
        let (tx_cmd, rx_cmd) = mpsc::channel::<usize>();
        let th = thread::spawn(move || UdpLink::run(socket, rx_msg, rx_cmd, sink));
        
        Ok(Self{
            server_addr: addr.into(),
            tx_msg,
            tx_cmd,
            th,
        })
        
    }

    fn run(socket: UdpSocket, rx_msg: Receiver<usize>, rx_cmd: Receiver<usize>, sink: Sender<usize>) {
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
                    println!("serial got {}", msg);
                    let msg = format!("{}", msg);
                    for c in &clients {
                        let _ = socket.send_to(msg.as_bytes(), c);
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
                    if let Ok(n) = trans.put(&buffer[0..nb]) {
                        println!("cool {}", n);
                        sink.send(n).expect("Coordinator is down.");
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),

            };

            
            thread::sleep(Duration::from_millis(1));    
        }
    }



    
}

impl Link for UdpLink {
    fn send_msg(&self, t: usize) -> Result<()> {
        self.tx_msg.send(t)?;
        Ok(())
    }

    fn stop(self) {
        let _ = self.tx_cmd.send(0);
        self.th.join().unwrap();
    }
}
