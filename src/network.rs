use std::net::TcpStream;
use std::io::prelude::*;
use std::io;
use std::io::{BufReader , BufWriter};
use std::net::{/* IpAddr, Ipv4Addr, SocketAddr, */ ToSocketAddrs};

pub fn read_line(reader: &mut BufReader<&TcpStream>) {
    let mut msg = String::new();
    reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
    println!("{}", msg);
}

pub fn write_to_tcp_stream(writer: &mut BufWriter<&TcpStream>, message: &str) {
    let msg = format!("{}", message);
    writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
    writer.flush().unwrap();
}

pub fn get_connected_tcp_stream(address :&str ,port: u32) -> io::Result<TcpStream> {
    let mut addresses = format!("{}:{}", address, port).to_socket_addrs()?;
    match addresses.find(|x| (*x).is_ipv4()) {
        None => {
            eprintln!("Invalid Host:Port Number");
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid host or port number"));
        },
        Some(addr) => return TcpStream::connect(addr),
    }
}