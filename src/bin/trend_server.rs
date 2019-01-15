#![allow(unused_imports)]

extern crate chrono;
extern crate clap;
extern crate gp_daq;
extern crate serde_yaml;

use clap::{App, Arg, SubCommand};

use chrono::offset::Utc;
use std::env;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
//use std::sync::mpsc::channel as channel;

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
    let matches=App::new("GRANDproto Data Server")
        .version("0.9")
        .author("GU Junhua. jhgu@nao.cas.cn")
        .about("Receiving TrendMsgs from remote DAQ Boards")
        .arg(Arg::with_name("Server IP")
                 .short("a")
                 .long("ipaddr")
                 .value_name("IP address")
                 .required(true)
                 .takes_value(true)
                 .help("The address of the ports to be bined to, can be either 0.0.0.0 or the IP address of the interface card"))
        .arg(Arg::with_name("SLC port")
            .short("s")
            .long("slcport")
            .value_name("SLC port")
            .required(true)
            .takes_value(true)
            .help("The port to accept SLC messages")
        )
        .arg(Arg::with_name("Monitor port")
            .short("m")
            .long("monport")
            .value_name("Monitor port")
            .required(true)
            .takes_value(true)
            .help("The monitor of some send_msg command to receive Ack message")
        )
        .arg(Arg::with_name("Data port")
            .short("d")
            .long("dataport")
            .value_name("DATA port")
            .required(true)
            .takes_value(true)
            .help("The port to accept TrendData")
        )
        .arg(Arg::with_name("Text file")
            .short("t")
            .long("txt")
            .value_name("File name")
            .required(false)
            .takes_value(true)
            .help("Save received message to the text file ")
        )
        .arg(Arg::with_name("Bin file")
            .short("b")
            .long("bin")
            .value_name("File name")
            .takes_value(true)
            .required(false)
            .help("The file used to save only TrendData msg")
        )
        .arg(Arg::with_name("No Data in txt")
            .short("n")
            .long("nodatatxt")
            .required(false)
            .takes_value(false)
            .help("If this flag is given, then TrendMsg will not be saved to txt file")
        )
        .arg(Arg::with_name("verbose level")
            .short("v")
            .long("verbose")
            .required(false)
            .takes_value(true)
            .value_name("Verbose level")
            .help("Currently only one verbose level is available, so simply fill 1|0 and 0 stands for not verbose")
        )
        .get_matches();

    let verbose = matches
        .value_of("verbose level")
        .map_or(0, |v| v.parse::<u32>().expect("Invalid verbose value"));

    let monitor_port: u16 = matches
        .value_of("Monitor port")
        .unwrap()
        .parse()
        .expect("Invalid monitor port");
    let addr_slc: SocketAddr = format!(
        "{}:{}",
        matches.value_of("Server IP").unwrap(),
        matches.value_of("SLC port").unwrap()
    ).parse()
    .expect("Invalid slc port");

    let addr_data: SocketAddr = format!(
        "{}:{}",
        matches.value_of("Server IP").unwrap(),
        matches.value_of("Data port").unwrap()
    ).parse()
    .expect("Invalid data port");

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

    let yaml_file = matches.value_of("Text file").map(|fname| {
        Arc::new(Mutex::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(fname)
                .expect("cannot open text file"),
        ))
    });

    let yaml_file_data: Option<_> = if matches.is_present("No Data in txt") {
        None
    } else {
        yaml_file.as_ref().cloned()
    };

    let mut bin_file = matches
        .value_of("Bin file")
        .map(|fname| File::create(fname).expect("cannot open bin file"));

    bin_file.iter_mut().for_each(|f| {
        let fh = FileHeader::new();
        fh.write_to(f);
    });

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

                yaml_file.as_ref().and_then(|f|{
                    let _=f.lock().and_then(|mut f|{
                        serde_yaml::to_writer(&mut *f, &v).expect("write failed");
                        writeln!(f).unwrap();
                        Ok(())
                    });
                    Some(())
                });
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
                if verbose > 0 {
                    eprint!(".");
                }
                let ev = Event::from_trend_data(&content, &payload);
                bin_file.iter_mut().for_each(|f| {
                    ev.write_to(f);
                });
                let mut v = msg.to_yaml();
                add_source_info(&mut v, &now, &ip[..]);
                //tx_data.send(v).expect("send err3");

                yaml_file_data.as_ref().and_then(|f| {
                    let _ = f.lock().map(|mut f| {
                        serde_yaml::to_writer(&mut *f, &v).expect("write failed");
                        writeln!(f).unwrap();
                        Some(())
                    });
                    Some(())
                });
            }
            ref msg => {
                eprintln!("Warning: only data msgs are expected to be received through data port");
                let mut v = msg.to_yaml();
                add_source_info(&mut v, &now, &ip[..]);
                //tx_data.send(v).expect("send err4");
                yaml_file_data.as_ref().and_then(|f| {
                    let _ = f.lock().map(|mut f| {
                        serde_yaml::to_writer(&mut *f, &v).expect("write failed");
                        writeln!(f).unwrap();
                        Some(())
                    });
                    Some(())
                });
            }
        }
        //msg.write_to_txt(&mut txt_file, &now).unwrap();
    }));
    let th_slc = thread::spawn(move || server_slc.run());
    let th_data = thread::spawn(move || server_data.run());

    th_slc.join().unwrap();
    th_data.join().unwrap();
}
