//
//use std::cmp::PartialEq;
mod link;
mod transport;
mod serial_link;
mod udp_link;

use std::{thread, time::Duration, sync::mpsc};
use anyhow::Result;
use link::{LinkMessage};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use clap::{Arg, App};
use std::net::{UdpSocket};


fn main() -> Result<()> {


    let matches = App::new("Serduck")
        .version("0.1.0")
        .about("UDP/Serial message server.")
        .arg(Arg::with_name("serial port")
                 .required(true)
                 .short("s")
                 .long("serial")
                 .takes_value(true)
                 .help("<port>"))
        .arg(Arg::with_name("serial baudrate")
                 .required(true)
                 .short("b")
                 .long("baudrate")
                 .takes_value(true)
                 .help("<baudrate>"))
        .arg(Arg::with_name("udp")
                 .required(true)
                 .short("u")
                 .long("udp")
                 .takes_value(true)
                 .help("<addr>:<port>"))
        .arg(Arg::with_name("transport")
                 .short("t")
                 .long("transport")
                 .takes_value(true)
                 .help("<duck|xbee>"))
        .get_matches();

    

    let ser = matches.value_of("serial port").unwrap();
    let baud = matches.value_of("serial baudrate").unwrap();
    let udp = matches.value_of("udp").unwrap();
    let baud: u32 = baud.parse()?;




    let (tx_msg_serial, rx_msg_serial) = mpsc::channel::<LinkMessage>();
    let (tx_msg_udp, rx_msg_udp) = mpsc::channel::<LinkMessage>();
    let (tx_cmd_serial, rx_cmd_serial) = mpsc::channel::<usize>();
    let (tx_cmd_udp, rx_cmd_udp) = mpsc::channel::<usize>();




    let serial = serialport::new(ser.clone(), baud)
        .timeout(Duration::from_millis(1))
        .open()?;

    let th_serial = thread::spawn(move || serial_link::run(serial, rx_msg_serial, rx_cmd_serial, tx_msg_udp));


    let socket = UdpSocket::bind(udp)?;
    socket.set_read_timeout(Some(Duration::from_millis(1))).expect("UDP set timeout failed");

    let th_udp = thread::spawn(move || udp_link::run(socket, rx_msg_udp, rx_cmd_udp, tx_msg_serial));




    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(1));
    }

    println!("stopping..........");
    tx_cmd_serial.send(0).unwrap();
    tx_cmd_udp.send(0).unwrap();
    th_serial.join().unwrap();
    th_udp.join().unwrap();

    

    Ok(())
}
