
use std::{io, thread, time::Duration};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use serialport::{SerialPort};
use crate::transport::Transport;
use crate::link::{LinkMessage};

pub fn run(mut serial: Box<dyn SerialPort>, rx_msg: Receiver<LinkMessage>, rx_cmd: Receiver<usize>, sink: Sender<LinkMessage>) {
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
                let buf = Transport::encode(&msg);
                match serial.write(&buf) {
                    Ok(n) if n == buf.len() => {

                    },
                    Ok(n) => {
                        println!("{} bytes written to serial, but buffer was {} bytes long!", n, buf.len());
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            },
            _ => {}
        }

        let mut buffer: [u8; 50] = [0; 50];
        match (*serial).read(&mut buffer) {
            Ok(nb) => {
                if let Ok(msg) = trans.put(&buffer[0..nb]) {
                    #[cfg(feature = "proto_debug")]
                    println!("rcv : {:?}", msg.to_proto().unwrap());
                    sink.send(msg).expect("Coordinator is down.");
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),

        };

        
        thread::sleep(Duration::from_micros(10));    
    }
}

