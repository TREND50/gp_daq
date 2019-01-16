#![allow(unused_imports)]

extern crate chrono;
extern crate clap;
extern crate gp_daq;
extern crate serde_yaml;

use clap::{App, Arg, SubCommand};
use std::str;
use chrono::offset::Utc;
use std::env;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
//use std::sync::mpsc::channel as channel;
use serde_yaml::{from_reader, from_str, Value};


use std::thread;

use gp_daq::io::event_file::{Event, FileHeader};
use gp_daq::io::yaml::YamlIOable;
use gp_daq::msg_def::msgcont::Ack_;
use gp_daq::msg_def::TrendMsg;
use gp_daq::net::client::send_msg;
use gp_daq::net::server::TrendServer;
use gp_daq::utils::add_source_info;


fn main(){
    let matches=App::new("Filter events in a yaml data file according to given trigger pattern")
        .version("0.9")
        .author("GU Junhua. jhgu@nao.cas.cn")
        .about("")
        .arg(Arg::with_name("input_file")
            .short("i")
            .long("input")
            .takes_value(true)
            .value_name("File name")
            .required(true)
            .help("Input yaml file name"))
        .arg(Arg::with_name("output_file")
            .short("o")
            .long("output")
            .takes_value(true)
            .value_name("File name")
            .required(true)
            .help(""))
        .arg(Arg::with_name("pattern")
            .short("p")
            .long("pattern")
            .takes_value(true)
            .value_name("pattern")
            .required(true)
            .help("The pattern")
        ).get_matches();

    let mut infile =
        File::open(matches.value_of("input_file").unwrap()).expect("Cannot open file");

    let mut outfile = OpenOptions::new().create(true).append(true)
        .open(matches.value_of("output_file").unwrap()).expect("cannot create output file");


        matches.value_of("output_file").map(|fname| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(fname)
            .expect("cannot open text file")
    });

    let pattern=matches.value_of("pattern").unwrap().parse::<u16>().expect("invalid pattern");

    let mut bytes = Vec::new();
    infile.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    //let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    for s in msg_str.split("---") {
        let _ = from_str::<Value>(s).map(|d| {
            //println!("{:?}", v);
            let v=&d["trig_pattern"];
            if let Some(p)=v.as_u64(){
                if p==pattern as u64{
                    serde_yaml::to_writer(&mut outfile, &d).expect("write failed");
                    writeln!(outfile).unwrap();
                }
            }
        });
    }

}