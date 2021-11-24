use std::thread::JoinHandle;
use std::{io, thread, time::Duration};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, SendError};
use serialport::{SerialPort};
use crate::transport::Transport;
//use std::error::Error;
use anyhow::Result;


// pub trait Link {
//     fn start(&mut self)-> Result<()>;
//     fn stop();
//     fn sendMsg(msg: usize);
// }

#[derive(Clone)]
pub struct SerialLinkConfig {
    pub port: String,
    pub baudrate: u32,
    pub timeout: u64,
}

pub struct SerialLink {
    pub config: SerialLinkConfig,
    tx: Sender<usize>,
    th: JoinHandle<()>
}


impl SerialLinkConfig {

    pub fn start(self, sink: Sender<usize>) -> Result<SerialLink>{
        let serial =
            serialport::new(self.port.clone(), self.baudrate)
            .timeout(Duration::from_millis(self.timeout))
            .open()?;


        let (tx, rx) = mpsc::channel::<usize>();
        let th = thread::spawn(move || SerialLink::run(serial, rx, sink));
        
        Ok(SerialLink{
            config: self,
            th,
            tx
        })
        
    }
}


impl SerialLink {

    fn run(mut serial: Box<dyn SerialPort>, rx: Receiver<usize>, sink: Sender<usize>) {
        println!("Bonjour!");

        let mut trans = Transport::new();

        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break;
                }
                Err(TryRecvError::Empty) => {}
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


    pub fn join(self){
        self.th.join().unwrap();
    }

    pub fn send_msg(&self, t: usize) -> Result<()> {
        self.tx.send(t)?;
        Ok(())
    }
    


}

