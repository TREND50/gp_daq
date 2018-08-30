extern crate gp_daq;
extern crate serde_yaml;
use gp_daq::cfg::YamlIOable;
use gp_daq::msgcont::Daq;
use serde_yaml::{from_reader, to_writer, Value};
use std::convert::From;
use std::fs::File;
use std::io::Read;
use std::env;

use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

fn main() {

    let mut f = File::open(env::args().nth(1).unwrap()).unwrap();
    let addr=env::args().nth(2).unwrap();
    let v: Value = from_reader(&mut f).unwrap();
    let daq1 = gp_daq::msg::TrendMsg::from_yaml(&v);

    let data = daq1.to_byte_vec();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(&data[..], addr);
}
