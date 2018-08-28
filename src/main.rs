extern crate grandproto_rs;

use grandproto_rs::msgcont::DaqContent;

fn main() {
    let mut my=DaqContent::<[u32;2]>([0x00, 0x00]);
    my.set_daq_on(1);
    my.set_cal_on(1);
    my.set_offst(0xff);
    my.set_le(1);
    my.set_att2(0x22);
    println!("{}",my.daq_on());
    //println!("{}", std::mem::size_of::<DaqContent<[u32;2]>>());
    println!("{}", my.0[1]);
}
