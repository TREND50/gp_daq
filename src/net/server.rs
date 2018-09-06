use super::super::msg_def::msg::TrendMsg;

use std;
use std::net::{SocketAddr, UdpSocket};

pub struct TrendServer {
    socket: UdpSocket,
    handlers: Vec<Box<FnMut(&TrendMsg, std::net::SocketAddr) -> ()>>,
}

impl TrendServer {
    pub fn register_handler(&mut self, h: Box<FnMut(&TrendMsg, std::net::SocketAddr) -> ()>) {
        self.handlers.push(h);
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = vec![0_u8; 65536];
            let (s, addr) = self.socket.recv_from(&mut buf[..]).unwrap();
            assert!(s <= buf.len());
            unsafe { buf.set_len(s) };
            //println!("{}", buf.len());

            match TrendMsg::from_byte_vec(buf) {
                Some(ref msg) => {
                    for h in &mut self.handlers {
                        h(&msg, addr);
                    }
                }
                _ => (),
            }
        }
    }

    pub fn new(addr: SocketAddr) -> Self {
        TrendServer {
            socket: UdpSocket::bind(&addr).unwrap(),
            handlers: Vec::new(),
        }
    }
}
