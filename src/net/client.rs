use super::super::msg_def::TrendMsg;
use super::server::TrendServer;
use etherparse::PacketBuilder;
use pcap::Error;
use pcap::{Capture, Device};
use std;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

const TIMEOUT: u32 = 500000000;

pub fn send_msg(
    addr: impl ToSocketAddrs + Send + 'static,
    msg: TrendMsg,
    monitor_port: Option<u16>,
) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed1");
    let data = msg.to_byte_vec();

    if let Some(p) = monitor_port {
        let msg_type = msg.type_code();
        let mut server = TrendServer::new(format!("0.0.0.0:{}", p).parse().unwrap());

        let j = std::thread::spawn(move || {
            std::thread::sleep(Duration::new(0, TIMEOUT));
            let _ = socket.send_to(&data[..], addr).expect("send data failed");
        });
        if let Some(msg) = server.wait_for(Some(Duration::new(1, 0))) {
            match msg {
                TrendMsg::Ack { ref content } if content.msg_ack() == msg_type as u16 => {
                    println!("Corresponding ack received");
                }
                TrendMsg::Ack { ref content } if content.msg_ack() != msg_type as u16 => {
                    println!("Warning:Ack received but type code mismatch");
                }
                _ => {
                    println!("Warning:Something received, but not Ack");
                }
            }
        } else {
            println!("No ack recived");
        }
        let _ = j.join();
    } else {
        let _ = socket.send_to(&data[..], addr).expect("send data failed");
    }
}

pub fn send_by_raw(
    dev: Device,
    dst_mac: [u8; 6],
    src_mac: [u8; 6],
    src_addr: impl ToSocketAddrs + Send + 'static,
    addr: impl ToSocketAddrs + Send + 'static,
    msg: TrendMsg,
    monitor_port: Option<u16>,
) {
    let (dst_ip, dst_port) = if let SocketAddr::V4(addr_v4) = addr
        .to_socket_addrs()
        .expect("not a valid addr")
        .next()
        .expect("no address get")
    {
        (addr_v4.ip().octets(), addr_v4.port())
    } else {
        panic!();
    };
    let (src_ip, src_port) = if let SocketAddr::V4(addr_v4) = src_addr
        .to_socket_addrs()
        .expect("not a valid addr")
        .next()
        .expect("no address get")
    {
        (addr_v4.ip().octets(), addr_v4.port())
    } else {
        panic!();
    };

    let mut cap = Capture::from_device(dev).unwrap().open().unwrap();
    let builder = PacketBuilder::ethernet2(src_mac, dst_mac)
        .ipv4(src_ip, dst_ip, 255)
        .udp(src_port, dst_port);
    let payload = msg.to_byte_vec();
    let mut data = Vec::with_capacity(builder.size(payload.len()));
    builder.write(&mut data, &payload);

    if let Some(p) = monitor_port {
        let msg_type = msg.type_code();
        let mut server = TrendServer::new(format!("0.0.0.0:{}", p).parse().unwrap());

        let j = std::thread::spawn(move || {
            std::thread::sleep(Duration::new(0, TIMEOUT));
            let _ = cap.sendpacket(&data[..]).expect("send data failed");
        });
        if let Some(msg) = server.wait_for(Some(Duration::new(1, 0))) {
            match msg {
                TrendMsg::Ack { ref content } if content.msg_ack() == msg_type as u16 => {
                    println!("Corresponding ack received");
                }
                TrendMsg::Ack { ref content } if content.msg_ack() != msg_type as u16 => {
                    println!("Warning:Ack received but type code mismatch");
                }
                _ => {
                    println!("Warning:Something received, but not Ack");
                }
            }
        } else {
            println!("No ack recived");
        }
        let _ = j.join();
    } else {
        let _ = cap.sendpacket(&data[..]).expect("send data failed");
    }
}
