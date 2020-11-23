#![allow(unused_imports)]

extern crate chrono;
extern crate clap;
extern crate gp_daq;
extern crate serde_yaml;

use chrono::offset::Utc;
use clap::{App, Arg, Subcommand};
use std::env;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::str;
use std::sync::{Arc, Mutex};
//use std::sync::mpsc::channel as channel;
use serde_yaml::{from_reader, from_str, Value};

use gp_daq::io::event_file::{Event, FileHeader};
use gp_daq::io::yaml::YamlIOable;
use gp_daq::msg_def::msgcont::Ack_;
use gp_daq::msg_def::TrendMsg;
use gp_daq::net::client::send_msg;
use gp_daq::net::server::TrendServer;
use gp_daq::utils::add_source_info;
use std::thread;

fn str2pattern(s: &str) -> Option<u64> {
    let r = if s.len() <= 2 {
        u64::from_str_radix(s, 10)
    } else if &s[0..2] == "0b" || &s[0..2] == "0B" {
        u64::from_str_radix(&s[2..], 2)
    } else if &s[0..2] == "0x" || &s[0..2] == "0X" {
        u64::from_str_radix(&s[2..], 16)
    } else {
        u64::from_str_radix(s, 10)
    };
    match r {
        Ok(x) => Some(x),
        _ => None,
    }
}

fn main() {
    let matches = App::new("Filter events in a yaml data file according to given trigger pattern")
        .version("0.9")
        .author("GU Junhua. jhgu@nao.cas.cn")
        .about("")
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input")
                .takes_value(true)
                .value_name("File name")
                .required(true)
                .about("Input yaml file name"),
        ).arg(
            Arg::new("output_file")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_name("File name")
                .required(true)
                .about(""),
        ).arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .takes_value(true)
                .value_name("pattern")
                .required(true)
                .about("The pattern"),
        ).get_matches();

    let mut infile = File::open(matches.value_of("input_file").unwrap()).expect("Cannot open file");

    let mut outfile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(matches.value_of("output_file").unwrap())
        .expect("cannot create output file");

    matches.value_of("output_file").map(|fname| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(fname)
            .expect("cannot open text file")
    });

    let pattern = str2pattern(matches.value_of("pattern").unwrap()).expect("Invalid pattern str");
    println!("filtering yaml according to pattern 0b{:b}", pattern);

    let mut bytes = Vec::new();
    infile.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    //let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    for s in msg_str.split("---") {
        let _ = from_str::<Value>(s).map(|d| {
            //println!("{:?}", v);
            let v = &d["trig_pattern"];
            if let Some(p) = v.as_u64() {
                if p == pattern as u64 {
                    serde_yaml::to_writer(&mut outfile, &d).expect("write failed");
                    writeln!(outfile).unwrap();
                }
            }
        });
    }
}
