extern crate gp_daq;
extern crate serde_yaml;
use gp_daq::cfg::YamlIOable;
use std::io::Read;
use std::str;
//use gp_daq::msgcont::Daq;
use serde_yaml::{from_reader, from_str,Value};

use std::fs::File;

use std::env;

use std::net::{UdpSocket};

fn main() {

    let mut f = File::open(env::args().nth(1).unwrap()).unwrap();
    let addr=env::args().nth(2).unwrap();

    let mut bytes=Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    let msg_str=str::from_utf8(&bytes).unwrap().to_string();

    for s in msg_str.split("---"){
        from_str::<Value>(s).map(|v|{
            println!("{:?}", v);
            let daq1 = gp_daq::msg::TrendMsg::from_yaml(&v);

            let data = daq1.to_byte_vec();
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            let _=socket.send_to(&data[..], addr.clone());
        });
    }
}
