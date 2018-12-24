#![allow(unused_imports)]

extern crate gp_daq;
extern crate interfaces;
extern crate pcap;
extern crate serde_yaml;

use gp_daq::net::net_err::NetErr;
use pcap::Device;
use std::io::Read;
use std::str;
//use gp_daq::msgcont::Daq;
use serde_yaml::{from_reader, from_str, Value};

use std::fs::File;

use std::env;

use std::net::UdpSocket;

use gp_daq::io::yaml::YamlIOable;
use gp_daq::msg_def::msg::TrendMsg;
use gp_daq::msg_def::msgcont::IntReg_;
use gp_daq::net::client::send_by_raw;
use gp_daq::net::client::send_msg;

fn u642mac(d: u64) -> [u8; 6] {
    let mut result = [0_u8; 6];
    for i in 0..6 {
        result[i] = (d >> ((5 - i) * 8) & 0xff) as u8;
    }
    //println!("{}\n{:?}", d, result);
    result
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: {} <iface> <yaml file> <port> <monitor port>",
            args[0]
        );
        return;
    }

    let port: u16 = args[3].parse().unwrap();
    let mac_addr = interfaces::Interface::get_by_name(args[1].as_str())
        .expect("cannot enumerate iface")
        .expect("no such a interface")
        .hardware_addr()
        .expect("get mac addr failed")
        .as_bytes()
        .to_vec();
    println!("host mac: {:x?}", mac_addr);

    let mut f = File::open(env::args().nth(2).expect(&format!(
        "Usage: {} <yaml> <addr:port>",
        env::args().nth(0).unwrap()
    )))
    .expect("Cannot open file");

    let monitor_port: u16 = args[4].parse().expect("invalid monitor port");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).expect("Cannot read file");
    let msg_str = str::from_utf8(&bytes).unwrap().to_string();
    //let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    for s in msg_str.split("---") {
        let _ = from_str::<Value>(s).map(|v| {
            //println!("{:?}", v);
            let msg = gp_daq::msg_def::TrendMsg::from_yaml(&v);
            //send_msg(addr.clone(), msg, Some(monitor_port));
            if let TrendMsg::IntReg { .. } = msg {
                let srv_mac1=u642mac(v["srv_mac1"].as_u64().expect("get host mac1 error"));
                let srv_mac2=u642mac(v["srv_mac2"].as_u64().expect("get host mac2 error"));
                if srv_mac1.iter().zip(mac_addr.iter()).any(|(&a,&b)|{a!=b}){
                    eprintln!("*********WARNING!!!*********");
                    eprintln!("Warning actual host mac addr mismatches the src_mac1 in cfg file: {:x?} vs {:x?}", mac_addr, srv_mac1);
                }
                if srv_mac2.iter().zip(mac_addr.iter()).any(|(&a,&b)|{a!=b}){
                    eprintln!("*********WARNING!!!*********");
                    eprintln!("Warning actual host mac addr mismatches the src_mac2 in cfg file: {:x?} vs {:x?}", mac_addr, srv_mac2);
                }
                //println!("{:x?}", v["srv_mac1"].as_sequence().unwrap().iter().map(|x|{x.as_u64().unwrap()}).collect::<Vec<_>>());
                //println!("{:x?}", v["srv_mac2"].as_sequence().unwrap().iter().map(|x|{x.as_u64().unwrap()}).collect::<Vec<_>>());
                let bmac = u642mac(v["board_mac"].as_u64().unwrap());
                let bip: Vec<u8> = v["board_ip"]
                    .as_sequence()
                    .map(|x| x.iter().map(|x| x.as_u64().unwrap() as u8).collect())
                    .unwrap();
                let srv_ip1: Vec<u8> = v["srv_ip1"]
                    .as_sequence()
                    .map(|x| x.iter().map(|x| x.as_u64().unwrap() as u8).collect())
                    .unwrap();
                let _srv_ip2: Vec<u8> = v["srv_ip2"]
                    .as_sequence()
                    .map(|x| x.iter().map(|x| x.as_u64().unwrap() as u8).collect())
                    .unwrap();
                let _port1 = v["port1"].as_u64().unwrap() as u16;
                let _port2 = v["port2"].as_u64().unwrap() as u16;
                let mut src_mac = [0; 6];
                for i in 0..6 {
                    src_mac[i] = mac_addr[i];
                }
                let dev = Device {
                    name: args[1].to_string(),
                    desc: None,
                };
                println!("setting board {:x?}'s IP address to be {:?}", bmac, bip);
                let _ = send_by_raw(
                    dev,
                    bmac,
                    src_mac,
                    std::net::SocketAddrV4::new(
                        std::net::Ipv4Addr::new(srv_ip1[0], srv_ip1[1], srv_ip1[2], srv_ip1[3]),
                        0,
                    ),
                    std::net::SocketAddrV4::new(
                        std::net::Ipv4Addr::new(bip[0], bip[1], bip[2], 0xff),
                        port,
                    ),
                    msg.clone(),
                    None,
                );
                let mut intreg = IntReg_([0; 11]);
                intreg.set_write(0);
                match send_msg(
                    (
                        std::net::Ipv4Addr::new(bip[0], bip[1], bip[2], bip[3]),
                        port,
                    ),
                    TrendMsg::IntReg { content: intreg },
                    Some(monitor_port),
                ) {
                    Ok(..) => eprintln!("Ack received"),
                    Err(x) => eprintln!("{:?}", x),
                }
            } else {
                println!("not intreg, skip");
            }
        });
    }
}
