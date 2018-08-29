#![recursion_limit="128"]
 #![feature(concat_idents)]
#[macro_use]
extern crate bitfield;


extern crate serde_yaml;


pub mod msgcont;
pub mod msg;
pub mod cfg;