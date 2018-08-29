extern crate grandproto_rs;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;

use serde_yaml::{from_reader, Value};
use std::convert::From;
use grandproto_rs::msgcont::Daq;

use grandproto_rs::cfg::*;
fn main() {
    let node:Value=from_reader(File::open("a.yml").unwrap()).unwrap();
    println!("{:?}", node);
    let bb=load_vec_u64(&node, "aa").unwrap();
    println!("{:?}", bb);
}
