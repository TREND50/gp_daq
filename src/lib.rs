#![recursion_limit = "128"]
#![feature(concat_idents)]
#[macro_use]
extern crate bitfield;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;



pub mod cfg;
pub mod msg;
pub mod msgcont;
pub mod io;
pub mod server;
pub mod event_file;