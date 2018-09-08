use std;
use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
use std::str::FromStr;
use tokio;
use super::super::msg_def::TrendMsg;
use super::server::TrendServer;


const TIMEOUT:u32=500;

pub fn send_msg(addr:impl ToSocketAddrs, msg:TrendMsg, monitor_port:Option<u16>){
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed1");
    let data = msg.to_byte_vec();

    if let Some(p)=monitor_port{
        let msg_type=msg.type_code();
        let mut server=TrendServer::new(format!("0.0.0.0:{}", p).parse().unwrap());
        server.register_handler(Box::new(move |msg,addr|{
            if let TrendMsg::Ack {content}=msg{
                if content.msg_ack() as u32==msg_type{
                    println!("Ack received and type code is OK");
                }else{
                    println!("Ack received but type code is not OK");
                }
            }else{
                println!("something received but not Ack");
            }
        }));


        let _ = socket
            .send_to(&data[..], addr)
            .expect("send data failed");
        server.wait();
    }
    else{
        let _ = socket
            .send_to(&data[..], addr)
            .expect("send data failed");
    }

}