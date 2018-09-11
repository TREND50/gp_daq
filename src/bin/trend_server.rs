#![allow(unused_imports)]

extern crate gp_daq;
extern crate serde_yaml;
extern crate chrono;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;


use chrono::offset::Utc;

use gp_daq::io::yaml::YamlIOable;
use gp_daq::io::event_file::{Event, FileHeader};
use gp_daq::msg_def::msgcont::Ack_;
use gp_daq::msg_def::TrendMsg;
use gp_daq::net::client::send_msg;
use gp_daq::net::server::TrendServer;

//deprecated
//use gp_daq::io::txt;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <addr:port> <monitor port> [out file prefix]",
            args[0]
        );
        return;
    }

    let monitor_port: u16 = args[2].parse().expect("invalid monitor port");

    let mut server = TrendServer::new(args[1].parse().expect("invalid port"));
    server.register_handler(Box::new(|a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    server.register_handler(Box::new(move |a: &TrendMsg, _b| {
        if let TrendMsg::Ack { content } = a {
            println!("forwarding ack");
            send_msg(
                format!("127.0.0.1:{}", monitor_port),
                TrendMsg::Ack {
                    content: Ack_([content.0[0], content.0[1]]),
                },
                None,
            );
        }
    }));

    if args.len() >= 4 {
        let file_prefix = args[3].clone();
        let mut yaml_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_prefix + ".yaml")
            .expect("cannot open file");
        /*
        let file_prefix = args[3].clone();
        let mut txt_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_prefix + ".txt")
            .expect("cannot open file");
        */
        let file_prefix = args[3].clone();
        let mut bin_file = File::create(file_prefix + ".bin").unwrap();
        let fh = FileHeader::new();
        fh.write_to(&mut bin_file);

        server.register_handler(Box::new(move |msg, socket| {
            let now=Utc::now();

            let ip:Vec<i64>=
            if let std::net::SocketAddr::V4(x)=socket{
                x.ip().octets().iter().map(|&x|{x as i64}).collect()
            }
            else{
                panic!("Ipv6 is not supported")
            };
            match msg {
                &TrendMsg::Data {
                    ref content,
                    ref payload,
                } => {
                    let ev = Event::from_trend_data(&content, &payload);
                    ev.write_to(&mut bin_file);
                    let mut v = msg.to_yaml();
                    v["received_timestamp"]=From::from(vec![now.timestamp(),now.timestamp_subsec_nanos() as i64]);
                    v["received_timestamp_str"]=From::from(now.to_string());
                    v["source_ip"]=From::from(ip.clone());
                    serde_yaml::to_writer(&mut yaml_file, &v).expect("write failed");
                    write!(yaml_file, "\n");
                }
                &TrendMsg::Ack {..}=>(),
                &ref msg => {
                    let mut v = msg.to_yaml();
                    v["received_timestamp"]=From::from(vec![now.timestamp(),now.timestamp_subsec_nanos() as i64]);
                    v["received_timestamp_str"]=From::from(now.to_string());
                    v["source_ip"]=From::from(ip.clone());
                    serde_yaml::to_writer(&mut yaml_file, &v).expect("write failed");
                    write!(yaml_file, "\n");
                },
            }
            //msg.write_to_txt(&mut txt_file, &now).unwrap();
        }));
    }
    server.run();
}
