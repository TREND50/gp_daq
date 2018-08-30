extern crate gp_daq;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;

use gp_daq::msg::TrendMsg;
use gp_daq::msgcont::Daq;
use serde_yaml::{from_reader, to_writer, Value};
use std::convert::From;

use gp_daq::cfg::*;
fn main() {
    let mut f = File::open("a.yaml").unwrap();
    let v: Value = from_reader(&mut f).unwrap();
    let daq1 = gp_daq::msg::TrendMsg::from_yaml(&v);
    println!("{:?}", daq1);
    if let gp_daq::msg::TrendMsg::Daq { ref content } = daq1 {
        println!("{:?}", content);
    }
    let xx = daq1.to_byte_vec();
    println!("{}", xx.len());
    let yy = TrendMsg::from_byte_vec(xx).unwrap();
    println!("{:?}", yy);
}
