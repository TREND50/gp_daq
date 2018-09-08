#![allow(unused_imports)]

extern crate gp_daq;
extern crate serde_yaml;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

use gp_daq::io::cfg::YamlIOable;
use gp_daq::io::event_file::{Event, FileHeader};
use gp_daq::msg_def::TrendMsg;
use gp_daq::net::server::TrendServer;
use gp_daq::net::client::send_msg;
use gp_daq::msg_def::msgcont::Ack_;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <addr:port> <monitor port> [out file prefix]", args[0]);
        return;
    }

    let monitor_port:u16=args[2].parse().expect("invalid monitor port");

    let mut server = TrendServer::new(args[1].parse().expect("invalid port"));
    server.register_handler(Box::new(|a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    server.register_handler(Box::new(move |a:&TrendMsg, b|{
        if let TrendMsg::Ack {content}=a{
            println!("forwarding ack");
            send_msg(format!("127.0.0.1:{}", monitor_port), TrendMsg::Ack {content:Ack_([content.0[0], content.0[1]])}, None);
        }
    }));

    if args.len() >= 4 {
        let file_prefix = args[3].clone();
        let mut txt_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_prefix + ".yaml")
            .expect("cannot open file");
        let file_prefix = args[3].clone();
        let mut bin_file = File::create(file_prefix + ".bin").unwrap();
        let fh = FileHeader::new();
        fh.write_to(&mut bin_file);

        server.register_handler(Box::new(move |a, b| {
            match a {
                &TrendMsg::Data {
                    ref content,
                    ref payload,
                } => {
                    let ev = Event::from_trend_data(&content, &payload);
                    ev.write_to(&mut bin_file);
                }
                _ => (),
            }
            let v = a.to_yaml();
            serde_yaml::to_writer(&mut txt_file, &v).expect("write failed");
            write!(txt_file, "\n");
        }));
    }
    server.run();
}
