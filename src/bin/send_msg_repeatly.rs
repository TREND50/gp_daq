#![allow(unused_imports)]

extern crate gp_daq;
extern crate serde_yaml;
extern crate clap;
use std::io::Read;
use std::str;
//use gp_daq::msgcont::Daq;

use clap::{App, Arg, Subcommand};

use serde_yaml::{from_reader, from_str, Value};

use std::fs::File;

use std::env;

use std::net::UdpSocket;

use gp_daq::io::yaml::YamlIOable;

use gp_daq::net::client::send_msg;

fn main() {
    let matches=App::new("GRANDproto Data Server")
        .version("0.9")
        .author("GU Junhua. jhgu@nao.cas.cn")
        .about("Receiving TrendMsgs from remote DAQ Boards")
        .arg(Arg::new("cfg_name")
            .short('i')
            .long("cfg")
            .value_name("cfg file name")
            .required(true)
            .takes_value(true)
            .about("cfg file to be sent")
        )
        .arg(Arg::new("addr_port")
            .short('a')
            .long("addr")
            .value_name("Address:port")
            .required(true)
            .takes_value(true)
            .about("Address:port")
        )
        .arg(Arg::new("count")
            .short('c')
            .long("cnt")
            .required(true)
            .takes_value(true)
            .value_name("counts")
            .about("counts")
        ).get_matches();


    let cfg_name=matches.value_of("cfg_name").unwrap();
    let addr=matches.value_of("addr_port").unwrap().to_string();
    let cnt=matches.value_of("count").unwrap().parse::<usize>().expect("parse count failed");

    let mut f = File::open(cfg_name).expect("Cannot open file");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    //let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");


    let msgs:Vec<_>=msg_str.split("---").map(|s|{
        from_str::<Value>(s).map(|v|{
            gp_daq::msg_def::TrendMsg::from_yaml(&v)
        })
    }).filter(|x|{x.is_ok()}).map(|x|{x.unwrap()}).collect();


    for _i in 0..cnt{
        for m in &msgs{
            send_msg(addr.clone(), m.clone(), None);
        }
    }
}
