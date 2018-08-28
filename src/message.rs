struct DaqContent{
    daq_on:u8,
    cal_on:u8, 
    rd_wr_plus:u8, 
    en_osc:u8, 
    cntrl_adc:u8, 
    offst:u8, 
    dis_pd:u8, 
    dis_lna:u8, 
    le:u8, 
    att1:u8, 
    att2:u8
}

struct TrigContent{
    st : u8,
    trg_en : u8,
    cntrl_adc : u16,
    th1m : u16,
    th1p : u16,
    th2m : u16,
    th2p : u16,
    th3m : u16,
    th3p : u16,
}

struct GpsContent{
    rwb:u8, 
    addr:u8,
    wrd:u8,
    data:Vec<u8>,
}

struct AdcContent{
    data:u16
}

struct IntRegContent{
    y:u8,
    data:Option<[u32;10]>
}

struct DataContent{
    ip:u32,
    ts2:u32,
    ts1_trigger:u8, 
    ts1_pps:u8,
    event_counter:u32,
    trig_pattern:u8,
    data:Vec<u16>
}

struct SlcContent{
    ip:u32, 
    vpower1:u16,
    vpower2:u16,

    vpower3:u16,
    vpower4:u16,
    
    vpower5:u16,
    vpower6:u16,

    th1m:u16,
    th1p:u16,

    th2m:u16,
    th2p:u16,

    th3m:u16,
    th3p:u16,

    temp:u16,
    total_trig_rate:u32,

    ch1p_trig_rate:u32,
    ch2p_trig_rate:u32,
    ch3p_trig_rate:u32,

    ch1m_trig_rate:u32,
    ch2m_trig_rate:u32,
    ch3m_trig_rate:u32,

    max_coarse:u32,
}

struct RdIntRegContent{
    ip:u32,
    board_mac:u32,
    board_ip:u32,
    dest_mac1_lsb:u32,
    dest_mac1_msb:u16,
    dest_ip1:u32,
    dest_mac2_lsb:u32,
    dest_mac2_msb:u16,
    dest_ip2:u32,
    dest_port1:u16,
    dest_port2:u32,
    serial_lsb:u32,
    serial_msb:u32,
}

struct AckContent{
    ip:u32,
    msg_ack:u16
}
pub enum TrendMsg{
    Daq{content:DaqContent},
    Trig{content:TrigContent},
    SlcReq,
    Gps{content:GpsContent},
    Adc{content:AdcContent},
    IntReg{content:IntRegContent},
    Data{content:DataContent},
    Slc{content:SlcContent},
    RdIntReg{content:RdIntRegContent},
    Ack{content:AckContent}
}

impl TrendMsg{
    pub fn type_code(&self)->u32{
        match self{
            &TrendMsg::Daq{..}=>0x5000,           
            &TrendMsg::Trig{..}=>0x5100,
            &TrendMsg::SlcReq=>0x5200,
            &TrendMsg::Gps{..}=>0x5300,
            &TrendMsg::Adc{..}=>0x5400,
            &TrendMsg::IntReg{..}=>0x5e00,
            &TrendMsg::Data{..}=>0x5a00,
            &TrendMsg::Slc{..}=>0x5b00,
            &TrendMsg::RdIntReg{..}=>0x5c00,
            &TrendMsg::Ack{..}=>0x5d00
        }
    }
}

