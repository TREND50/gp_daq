#![recursion_limit = "128"]
#![feature(concat_idents)]
#[macro_use]
extern crate bitfield;
extern crate bytes;
extern crate serde_yaml;
extern crate tokio;
extern crate chrono;

pub mod io;
pub mod msg_def;
pub mod net;
