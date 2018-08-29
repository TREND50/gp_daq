extern crate grandproto_rs;

use grandproto_rs::msgcont::Daq;

fn main() {
    let mut my=Daq::<[u32;2]>([0x00, 0x00]);
    my.set_daq_on(1);
    my.set_cal_on(1);
    my.set_offst(0xff);
    my.set_le(1);
    my.set_att2(0x22);
    println!("{}",my.daq_on());
    //println!("{}", std::mem::size_of::<Daq<[u32;2]>>());
    println!("{}", my.0[1]);
    println!("{}", my.0.len());
}
