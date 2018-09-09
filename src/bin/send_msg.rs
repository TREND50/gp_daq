#![allow(unused_imports)]

extern crate gp_daq;
extern crate serde_yaml;
use std::io::Read;
use std::str;
//use gp_daq::msgcont::Daq;
use serde_yaml::{from_reader, from_str, Value};

use std::fs::File;

use std::env;

use std::net::UdpSocket;

use gp_daq::io::cfg::YamlIOable;

use gp_daq::net::client::send_msg;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <cfg file> <addr:port> <monitor port>", args[0]);
        return;
    }

    let mut f = File::open(env::args().nth(1).expect(&format!(
        "Usage: {} <cfg> <addr:port>",
        env::args().nth(0).unwrap()
    ))).expect("Cannot open file");
    let addr = env::args().nth(2).expect("Invalid addr");

    let monitor_port: u16 = args[3].parse().expect("invalid monitor port");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    //let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    for s in msg_str.split("---") {
        let _ = from_str::<Value>(s).map(|v| {
            //println!("{:?}", v);
            let daq1 = gp_daq::msg_def::TrendMsg::from_yaml(&v);
            send_msg(addr.clone(), daq1, Some(monitor_port));
        });
    }
}
