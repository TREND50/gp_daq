extern crate gp_daq;
extern crate serde_yaml;
use std::fs::File;
use std::io::Read;
use serde_yaml::{from_reader, to_writer, Value};
use std::convert::From;
use gp_daq::msgcont::Daq;
use gp_daq::cfg::YamlIOable;

use std::net::{ToSocketAddrs, SocketAddr, UdpSocket};

fn main(){
    let mut f=File::open("a.yaml").unwrap();
    let v:Value=from_reader(&mut f).unwrap();
    let daq1=gp_daq::msg::TrendMsg::from_yaml(&v);
    
    let data=daq1.to_byte_vec();
    let socket=UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(&data[..], "10.11.0.36:1234");
}
