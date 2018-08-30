extern crate gp_daq;
extern crate serde_yaml;



use std::env;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::fs::File;
use std::io::Read;
use gp_daq::msg::TrendMsg;
use gp_daq::msgcont::Daq;
use serde_yaml::{from_reader, to_writer, Value};
use std::convert::From;
use gp_daq::cfg::*;
use gp_daq::server::TrendServer;


fn main() {
    let addr = env::args().nth(1).unwrap_or("0.0.0.0:1234".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    let mut server=TrendServer::new(addr);
    server.register_handler(Box::new(move |msg,_|{println!("{:?}", msg);}));
    server.run();
}
