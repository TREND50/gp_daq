#![allow(unused_imports)]

extern crate chrono;
extern crate gp_daq;
extern crate serde_yaml;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::SocketAddr;
use std::sync::mpsc::sync_channel;
use std::thread;
use chrono::offset::Utc;

use gp_daq::io::event_file::{Event, FileHeader};
use gp_daq::io::yaml::YamlIOable;
use gp_daq::msg_def::msgcont::Ack_;
use gp_daq::msg_def::TrendMsg;
use gp_daq::net::client::send_msg;
use gp_daq::net::server::TrendServer;
use gp_daq::utils::add_source_info;

//deprecated
//use gp_daq::io::txt;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 6 {
        eprintln!(
            "Usage: {} <addr> <slc port> <data port> <monitor port> <out file prefix>",
            args[0]
        );
        return;
    }

    let monitor_port: u16 = args[4].parse().expect("invalid monitor port");
    let addr_slc: SocketAddr = format!("{}:{}", args[1], args[2])
        .parse()
        .expect("invalid slc port");
    let addr_data: SocketAddr = format!("{}:{}", args[1], args[3])
        .parse()
        .expect("invalid slc port");

    let mut server_slc = TrendServer::new(addr_slc);
    server_slc.register_handler(Box::new(move |a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    let mut server_data = TrendServer::new(addr_data);
    server_slc.register_handler(Box::new(move |a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    server_slc.register_handler(Box::new(move |a: &TrendMsg, _b| {
        if let TrendMsg::Ack { content } = a {
            println!("forwarding ack");
            let _ = send_msg(
                format!("127.0.0.1:{}", monitor_port),
                TrendMsg::Ack {
                    content: Ack_([content.0[0], content.0[1]]),
                },
                None,
            );
        }
    }));

    let file_prefix = args[5].clone();
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


    let file_prefix = args[5].clone();
    let mut bin_file = File::create(file_prefix + ".bin").unwrap();
    let fh = FileHeader::new();
    fh.write_to(&mut bin_file);

    let (tx_slc, rx)=sync_channel(16);

    let tx_data=tx_slc.clone();


    server_slc.register_handler(Box::new(move |msg, socket| {
        let now = Utc::now();

        let ip: Vec<i64> = if let std::net::SocketAddr::V4(x) = socket {
            x.ip().octets().iter().map(|&x| i64::from(x)).collect()
        } else {
            panic!("Ipv6 is not supported")
        };
        match *msg {
            TrendMsg::Data {..} => {
                eprintln!("Warning: no data msg is expected to be received through slc port, simply ignore it");
                //let mut v = msg.to_yaml();
                //add_source_info(&mut v, &now, &ip[..]);
                //tx_slc.send(v).expect("send err1");
            }
            TrendMsg::Ack { .. } => (),
            ref msg => {
                let mut v = msg.to_yaml();
                add_source_info(&mut v, &now, &ip[..]);
                tx_slc.send(v).expect("send err2");
            }
        }
        //msg.write_to_txt(&mut txt_file, &now).unwrap();
    }));

    server_data.register_handler(Box::new(move |msg, socket| {
        let now = Utc::now();

        let ip: Vec<i64> = if let std::net::SocketAddr::V4(x) = socket {
            x.ip().octets().iter().map(|&x| i64::from(x)).collect()
        } else {
            panic!("Ipv6 is not supported")
        };
        match *msg {
            TrendMsg::Data {
                ref content,
                ref payload,
            } => {
                let ev = Event::from_trend_data(&content, &payload);
                ev.write_to(&mut bin_file);
                let mut v = msg.to_yaml();
                add_source_info(&mut v, &now, &ip[..]);
                tx_data.send(v).expect("send err3");
            }
            ref msg => {
                eprintln!("Warning: only data msgs are expected to be received through data port");
                let mut v = msg.to_yaml();
                add_source_info(&mut v, &now, &ip[..]);
                tx_data.send(v).expect("send err4");
            }
        }
        //msg.write_to_txt(&mut txt_file, &now).unwrap();
    }));
    let th_io=thread::spawn(move ||{
        loop {
            let v = rx.recv().expect("recv err");
            serde_yaml::to_writer(&mut yaml_file, &v).expect("write failed");
            writeln!(yaml_file).unwrap();
        }
    });
    let th_slc=thread::spawn(move||server_slc.run());
    let th_data=thread::spawn(move||server_data.run());

    th_io.join().expect("join err");
    th_slc.join().expect("join err");
    th_data.join().expect("join err");

}
