use std::net::{SocketAddr, UdpSocket};
use std::cmp::PartialEq;
mod link;

enum Client {
    UDP(SocketAddr),
    SERIAL,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UDP(l0), Self::UDP(r0)) => l0 == r0,
            _ => false,
        }
    }
}


fn main() -> std::io::Result<()> {
    let mut clients: Vec<Client> = Vec::new();
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let sp = link::SerialLink::new("/dev/ttyUSB0", 115200);



    loop
    {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        
        if !clients.iter().any(|sa| sa == &Client::UDP(src)) {
            clients.push(Client::UDP(src));
        }


        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();

        let _ = clients.iter().map(|c| match c {
            Client::UDP(addr) => socket.send_to(buf, &addr),
            _ => panic!(""),
            
        }).collect::<Vec<_>>();

        //socket.send_to(buf, &src)?;
    } // the socket is closed here
    Ok(())
}

