extern crate gp_daq;
extern crate tokio;
//use std::env;
use tokio::codec::Decoder;
use tokio::net::{UdpSocket,UdpFramed};
use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use tokio::prelude::Stream;
use tokio::prelude::Future;
use gp_daq::net::codec::MsgDecoder;

fn main() {
    let addr=SocketAddr::new(Ipv4Addr::new(0,0,0,0).into(),1234);
    let socket=UdpSocket::bind(&addr).unwrap();
    let socket=UdpFramed::new(socket, MsgDecoder{});
    let server=socket.for_each(|(msg, socket)|{
        println!("{:?}",msg);
        Ok(())})
        .map_err(|err|{});
    tokio::run(server);
}
