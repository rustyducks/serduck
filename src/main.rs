//
//use std::cmp::PartialEq;
mod link;
mod transport;
mod serial_link;
mod udp_link;

use std::{thread, time::Duration, sync::mpsc};
use anyhow::Result;
use serial_link::{SerialLinkConfig, SerialLink};
use udp_link::UdpLink;
use link::{Link, LinkMessage};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use anyhow::anyhow;
use clap::{Arg, App};


fn main() -> Result<()> {


    let matches = App::new("Serduck")
        .version("0.1.0")
        .about("UDP/Serial message server.")
        .arg(Arg::with_name("serial")
                 .short("s")
                 .long("serial")
                 .takes_value(true)
                 .help("<port>:<baudrate>"))
        .arg(Arg::with_name("udp")
                 .short("u")
                 .long("udp")
                 .takes_value(true)
                 .help("<addr>:<port>"))
        .get_matches();

    let mut links: Vec<Box<dyn Link>> = Vec::new();
    let mut errors : Vec<String> = Vec::new();


    let (tx, rx) = mpsc::channel::<LinkMessage>();
    

    if let Some(serials) = matches.values_of("serial") {
        for ser in serials {
            match add_serial(ser, tx.clone()) {
                Ok(sl) => links.push(Box::new(sl)),
                Err(e) => errors.push(e.to_string())
            }
        }
    }

    if let Some(udps) = matches.values_of("udp") {
        for udp in udps {
            match add_udp(udp, tx.clone()) {
                Ok(ul) => links.push(Box::new(ul)),
                Err(e) => errors.push(e.to_string())
            }
        }
    }

    for err in errors {
        println!("{}", err);
    }

    if links.len() == 0 {
        return Err(anyhow!("No link successfully started!"))
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


fn add_serial(serial_params: &str, tx: mpsc::Sender<LinkMessage>) -> Result<SerialLink> {
    let params: Vec<_> = serial_params.split(':').collect();
    if params.len() == 2 {
        let port = params[0];
        let baudrate: u32 = params[1].parse()?;
        let slc = SerialLinkConfig {
            port: port.into(),
            baudrate,
            timeout: 1,
        };

        match slc.start(tx) {
            Ok(sl) => Ok(sl),
            Err(e) => {
                Err(anyhow!("Error for {} : {:?}", serial_params, e))
            },
        }

    } else {
        Err(anyhow!("can't split \"{}\" as port:baudrate", serial_params))
    }
}

fn add_udp(udp_params: &str, tx: mpsc::Sender<LinkMessage>) -> Result<UdpLink> {
    match UdpLink::new(udp_params, tx, 1) {
        Ok(ul) => Ok(ul),
        Err(e) => {
            Err(anyhow!("Error for {} : {:?}", udp_params, e))
        },
    }
}
