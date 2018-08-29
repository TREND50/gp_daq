extern crate grandproto_rs;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;

use serde_yaml::{from_reader, to_writer, Value};
use std::convert::From;
use grandproto_rs::msgcont::Daq;

use grandproto_rs::cfg::*;
fn main() {
    
    let mut f=File::open("a.yaml").unwrap();
    let v:Value=from_reader(&mut f).unwrap();
    let daq1=grandproto_rs::msg::TrendMsg::from_yaml(&v);
    if let grandproto_rs::msg::TrendMsg::Daq{content}=daq1{
        println!("{:?}", content);
    }
}
