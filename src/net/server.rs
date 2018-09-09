use super::super::msg_def::msg::TrendMsg;

use super::codec::MsgDecoder;
use std;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
//use tokio::codec::Decoder;
use tokio::net::{UdpFramed, UdpSocket as TUdpSocket};
use tokio::prelude::Future;
use tokio::prelude::Stream;

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

    pub fn wait_for(&mut self, dt:Option<Duration>)->Option<TrendMsg>{
        let mut buf = vec![0_u8; 65536];
        self.socket.set_read_timeout(dt).expect("set timeout failed");
        if let Ok((s, _addr)) = self.socket.recv_from(&mut buf[..]){
            assert!(s <= buf.len());
            unsafe { buf.set_len(s) };
            //println!("{}", buf.len());
            TrendMsg::from_byte_vec(buf)
        }else{
            None
        }
    }

    pub fn new(addr: SocketAddr) -> Self {
        TrendServer {
            socket: UdpSocket::bind(&addr).expect(&format!("bind to addr {} failed", addr)),
            handlers: Vec::new(),
        }
    }
}

pub fn create_async_server(
    addr: SocketAddr,
    handler: impl FnMut((TrendMsg, SocketAddr)) -> Result<(), <UdpFramed<MsgDecoder> as Stream>::Error>,
) -> impl Future<Item = (), Error = ()> {
    println!("port={}", addr.port());
    UdpFramed::new(TUdpSocket::bind(&addr).expect("bind failed3"), MsgDecoder {})
        //.for_each(|(msg, _socket)| { Ok(())})
        .for_each(handler)
        .map_err(|_err| {})
}
