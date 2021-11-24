
use std::thread::JoinHandle;
use std::{io, thread, time::Duration};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use serialport::{SerialPort};
use crate::transport::Transport;
use anyhow::Result;
use crate::link::Link;

#[derive(Clone)]
pub struct SerialLinkConfig {
    pub port: String,
    pub baudrate: u32,
    pub timeout: u64,
}

pub struct SerialLink {
    pub config: SerialLinkConfig,
    tx_msg: Sender<usize>,
    tx_cmd: Sender<usize>,
    th: JoinHandle<()>
}


impl SerialLinkConfig {

    pub fn start(self, sink: Sender<usize>) -> Result<SerialLink> {
        let serial =
            serialport::new(self.port.clone(), self.baudrate)
            .timeout(Duration::from_millis(self.timeout))
            .open()?;


        let (tx_msg, rx_msg) = mpsc::channel::<usize>();
        let (tx_cmd, rx_cmd) = mpsc::channel::<usize>();
        let th = thread::spawn(move || SerialLink::run(serial, rx_msg, rx_cmd, sink));
        
        Ok(SerialLink{
            config: self,
            tx_msg,
            tx_cmd,
            th,
        })
        
    }
}


impl SerialLink {

    fn run(mut serial: Box<dyn SerialPort>, rx_msg: Receiver<usize>, rx_cmd: Receiver<usize>, sink: Sender<usize>) {
        let mut trans = Transport::new();

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
                    let ret = serial.write(msg.as_bytes());
                },
                _ => {}
            }

            let mut buffer: [u8; 50] = [0; 50];
            match (*serial).read(&mut buffer) {
                Ok(nb) => {
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

impl Link for SerialLink {
    fn send_msg(&self, t: usize) -> Result<()> {
        self.tx_msg.send(t)?;
        Ok(())
    }

    fn stop(self) {
        let _ = self.tx_cmd.send(0);
        self.th.join().unwrap();
    }
}
