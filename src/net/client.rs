use super::super::msg_def::TrendMsg;
use super::server::TrendServer;
use std;
use std::net::{ToSocketAddrs, UdpSocket};
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
