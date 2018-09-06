#![allow(unused_imports)]

extern crate gp_daq;
extern crate tokio;
//use std::env;
use gp_daq::net::codec::MsgDecoder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::codec::Decoder;
use tokio::net::{UdpFramed, UdpSocket};
use tokio::prelude::Future;
use tokio::prelude::Stream;

fn main() {
    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), 1234);
    let socket = UdpSocket::bind(&addr).unwrap();
    let socket = UdpFramed::new(socket, MsgDecoder {});
    let server = socket
        .for_each(|(msg, _socket)| {
            println!("{:?}", msg);
            Ok(())
        }).map_err(|_err| {});
    tokio::run(server);
}
