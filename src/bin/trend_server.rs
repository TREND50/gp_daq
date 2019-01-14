#![allow(unused_imports)]

extern crate chrono;
extern crate gp_daq;
extern crate serde_yaml;
use chrono::offset::Utc;
use std::env;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::SocketAddr;
use std::sync::mpsc::sync_channel;
use std::thread;

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
    //let args: Vec<_> = std::env::args().collect();
    if std::env::args().len() < 5 {
        eprintln!(
            "Usage: {} <addr> <slc port> <data port> <monitor port> [out file prefix]",
            args().nth(0).unwrap()
        );
        return;
    }

    let monitor_port: u16 = args()
        .nth(4)
        .unwrap()
        .parse()
        .expect("invalid monitor port");
    let addr_slc: SocketAddr = format!("{}:{}", args().nth(1).unwrap(), args().nth(2).unwrap())
        .parse()
        .expect("invalid slc port");
    let addr_data: SocketAddr = format!("{}:{}", args().nth(1).unwrap(), args().nth(3).unwrap())
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

    let mut yaml_file = args().nth(5).map(|file_prefix| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_prefix + ".yaml")
            .expect("cannot open file")
    });

    let mut bin_file = args()
        .nth(5)
        .map(|file_prefix| File::create(file_prefix + ".bin").unwrap());

    bin_file.iter_mut().for_each(|f| {
        let fh = FileHeader::new();
        fh.write_to(f);
    });

    let (tx_slc, rx) = sync_channel(16);

    let tx_data = tx_slc.clone();

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
                println!("aa");
                let ev = Event::from_trend_data(&content, &payload);
                bin_file.iter_mut().for_each(|f| {
                    ev.write_to(f);
                });
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
    thread::spawn(move || server_slc.run());
    thread::spawn(move || server_data.run());

    loop {
        let v = rx.recv().expect("recv err");
        yaml_file.iter_mut().for_each(|f| {
            serde_yaml::to_writer(&mut *f, &v).expect("write failed");
            writeln!(f).unwrap();
        });
    }
}
