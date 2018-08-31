extern crate gp_daq;
extern crate serde_yaml;

use gp_daq::cfg::YamlIOable;
use gp_daq::event_file::{Event, FileHeader};
use gp_daq::msg::TrendMsg;
use gp_daq::server::TrendServer;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    let args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port> [out file prefix]", args[0]);
        return;
    }

    let mut server = TrendServer::new(args[1].parse().unwrap());
    server.register_handler(Box::new(|a, b| {
        println!("recv from {:?}", b);
        println!("msg:\n{:?}", a);
    }));

    if args.len() >= 3 {
        let file_prefix = args[2].clone();
        let mut txt_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_prefix + ".yaml")
            .unwrap();
        let file_prefix = args[2].clone();
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
            serde_yaml::to_writer(&mut txt_file, &v);
            write!(txt_file, "\n");
        }));
    }
    server.run();
}
