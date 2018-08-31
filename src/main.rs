extern crate gp_daq;

//use std::env;

use gp_daq::event_file::*;
use std::fs::File;

fn main() {
    let fh = FileHeader::new().with_additional_header(vec![0, 1, 2]);
    let mut file = File::create("a.bin").unwrap();
    fh.write_to(&mut file);
    println!("{:?}", fh);
    let mut file = File::open("a.bin").unwrap();
    let fh1 = FileHeader::read_from(&mut file).unwrap();
    println!("{:?}", fh1);
    println!("{}", std::mem::size_of_val(&fh.basic_header));

    let mut event = Event::new(EventHeader::default());
    let lsh = LocalStationHeader::default();
    let ls = LocalStation::new(lsh, vec![0, 1], vec![0, 1, 2, 3, 4]);
    event.push_local_station(ls);
    println!("{:?}", event);

    let event_file = EventFile {
        header: fh,
        event_list: vec![event],
    };
    println!("{:?}", event_file);
    let mut file2 = File::create("b.bin").unwrap();
    event_file.write_to(&mut file2);
    let mut file2 = File::open("b.bin").unwrap();
    let ef = EventFile::read_from(&mut file2).unwrap();
    println!("{:?}", ef);
}
