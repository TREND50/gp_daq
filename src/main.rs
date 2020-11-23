#![allow(unused_imports)]

extern crate gp_daq;
extern crate tokio;
extern crate tokio_util;
extern crate futures;
//use std::env;
use gp_daq::net::codec::MsgDecoder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use tokio_util::codec::Decoder;
use tokio::net::{ UdpSocket};
//use tokio_util::udp::UdpFramed;
use futures::executor::block_on;

fn main() {
    /*
    let server1 = gp_daq::net::server::create_async_server(
        SocketAddr::from_str("0.0.0.0:1234").unwrap(),
        |r| {
            if let Ok((msg, _socket)) = r {
                println!("{:?}", msg);
            }
            futures::future::ready(())
        },
    );
    let server2 = gp_daq::net::server::create_async_server(
        SocketAddr::from_str("0.0.0.0:1235").unwrap(),
        |r| {
            if let Ok((msg, _socket)) = r {
                println!("{:?}", msg);
            }
            futures::future::ready(())
        },
    );

    let _j = std::thread::spawn(|| {

        //tokio::run(server1.join(server2).map(|_| {}));
        //block_on(server1.join(server2.join));
        block_on(async{futures::join!(server1, server2)});
    });
    println!("a");
    //tokio::run(server1);

    */
}
