#![allow(unused_imports)]

extern crate gp_daq;
extern crate serde_yaml;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

use gp_daq::io::event_file::{Event, FileHeader};
use gp_daq::io::yaml::YamlIOable;
use gp_daq::msg_def::msgcont;
use gp_daq::msg_def::TrendMsg;
use gp_daq::net::client::send_msg;
use gp_daq::net::server::TrendServer;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <addr:port> <controller slc port>", args[0]);
        return;
    }

    let mut server = TrendServer::new(args[1].parse().expect("invalid port"));
    let slc_port: u16 = args[2].parse().expect("Not a valid slc port");
    server.register_handler(Box::new(|a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    server.register_handler(Box::new(move |a, b| match a {
        &TrendMsg::Ack { .. } => {
            println!("ack");
        }
        m => {
            println!("{:?}", m);
            if let std::net::IpAddr::V4(ip4) = b.ip() {
                let ipu32: u32 = std::convert::From::from(ip4);
                let mut ack = msgcont::Ack_([0_u32, 0_u32]);
                ack.set_ip(ipu32);
                ack.set_msg_ack(m.type_code() as u16);
                let back_addr =
                    std::net::SocketAddr::V4(std::net::SocketAddrV4::new(ip4, slc_port));
                println!("ack sent");
                println!("{}", back_addr.port());
                let _ = send_msg(back_addr, TrendMsg::Ack { content: ack }, None);
            }
        }
    }));
    server.run();
}
