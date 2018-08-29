use either::*;
use msgcont::{self};

pub enum TrendMsg{
    Daq{content:msgcont::Daq<[u32;2]>},
    Trig{content:Either<msgcont::Trig1<[u32;4]>, msgcont::Trig2<[u32;1]>>},
    SlcReq,
    Gps{content:msgcont::Gps<[u32;1]>, data:Vec<u8>},
    Adc{content:msgcont::Adc<[u32;1]>},
    IntReg{content:Either<msgcont::IntReg1<[u32;1]>, msgcont::IntReg2<[u32;11]>>},
    Data{content:msgcont::Data<[u32;5]>, data:Vec<u16>},
    Slc{content:msgcont::Slc<[u32;16]>},
    RdIntReg{content:msgcont::RdIntReg<[u32;13]>},
    Ack{content:msgcont::Ack<[u32;2]>},
}


