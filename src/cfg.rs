use super::msg;
use super::msgcont;


use std::default::Default;
use serde_yaml::{Value, Mapping};

pub fn load_vec_u64(data: &Value, k: &str) -> Option<Vec<u64>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap()).collect())
}

pub fn load_vec_u32(data: &Value, k: &str) -> Option<Vec<u32>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap() as u32).collect())
}

pub fn load_vec_u16(data: &Value, k: &str) -> Option<Vec<u16>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap() as u16).collect())
}

pub fn load_vec_u8(data: &Value, k: &str) -> Option<Vec<u8>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap() as u8).collect())
}

pub fn load_str(data:&Value, k:&str)->Option<String>{
    data[k].as_str().map(|ref x| x.to_string())
}

pub fn load_u64(data: &Value, k: &str) -> Option<u64> {
    data[k].as_u64()
}

pub fn load_u32(data: &Value, k: &str) -> Option<u32> {
    data[k].as_u64().map(|x| x as u32)
}

pub fn load_u16(data: &Value, k: &str) -> Option<u16> {
    data[k].as_u64().map(|x| x as u16)
}

pub fn load_u8(data: &Value, k: &str) -> Option<u8> {
    data[k].as_u64().map(|x| x as u8)
}

pub fn store_u64(data:&mut Value, k:&str, v:u64){
    data[k]=From::from(v);
}

pub fn store_u32(data:&mut Value, k:&str, v:u32){
    data[k]=From::from(v);
}

pub fn store_u16(data:&mut Value, k:&str, v:u16){
    data[k]=From::from(v);
}

pub fn store_u8(data:&mut Value, k:&str, v:u8){
    data[k]=From::from(v);
}

macro_rules! yaml_io{
    ($(($name:ident, $setter:ident, $storer:ident, $loader:ident)),*)=>{
        fn from_yaml(cfg:&Value)->Self{
            let mut result:Self=Default::default();
            $(
            result.$setter($loader(cfg, stringify!($name)).unwrap_or_else(||{
                eprintln!("WARNING: {} not found, use 0", stringify!($name) );
                0
            }));
            )*
            result
        }

        fn to_yaml(&self)->Value{
            let mut result=Value::Mapping(Mapping::new());
            $(
                $storer(&mut result, stringify!($name), self.$name());
            )*
            result
        }
    }
}

pub trait YamlIOable{
    fn from_yaml(cfg:&Value)->Self;
    fn to_yaml(&self)->Value;
}

impl YamlIOable for msgcont::Daq<[u32;2]>{

    yaml_io![(daq_on,set_daq_on, store_u8, load_u8),
    (cal_on, set_cal_on, store_u8, load_u8),
    (rd_wr_plus, set_rd_wr_plus, store_u8, load_u8),
    (en_osc, set_en_osc, store_u8, load_u8),
    (cntrl_adc, set_cntrl_adc, store_u8, load_u8),
    (offst, set_offst, store_u16, load_u16),
    (dis_pd, set_dis_pd, store_u8, load_u8),
    (dis_lna, set_dis_lna, store_u8, load_u8),
    (le, set_le, store_u8, load_u8),
    (att1, set_att1, store_u8, load_u8),
    (att2, set_att2, store_u8, load_u8)
    ];
}


impl YamlIOable for msgcont::Trig<[u32;4]>{
    yaml_io![
    (st, set_st, store_u8, load_u8),
    (trg_en, set_trg_en, store_u8, load_u8),
    (cntrl_dac, set_cntrl_dac,store_u16, load_u16),
    (th1m, set_th1m, store_u16, load_u16),
    (th1p, set_th1p, store_u16, load_u16),
    (th2m, set_th2m, store_u16, load_u16),
    (th2p, set_th2p, store_u16, load_u16),
    (th3m, set_th3m, store_u16, load_u16),
    (th3p, set_th3p, store_u16, load_u16)
    ];
}

impl YamlIOable for msgcont::Gps<[u32;1]>{
    yaml_io![
        (rwb, set_rwb, store_u8, load_u8),
        (addr, set_addr, store_u8, load_u8),
        (wrd, set_wrd, store_u8, load_u8)
    ];
}

impl YamlIOable for msgcont::Adc<[u32;1]>{
    yaml_io![
        (data, set_data, store_u16, load_u16)
    ];
}

impl YamlIOable for msgcont::IntReg<[u32;11]>{
    yaml_io![
        (y, set_y, store_u8, load_u8),
        (board_mac, set_board_mac, store_u32, load_u32),
        (board_ip, set_board_ip,store_u32, load_u32),
        (dst_mac1, set_dst_mac1, store_u64, load_u64),
        (dst_ip1,set_dst_ip1,store_u32, load_u32),
        (dst_mac2, set_dst_mac2,store_u64, load_u64),
        (dst_ip2,set_dst_ip2,store_u32, load_u32),
        (port1, set_port1, store_u16, load_u16),
        (port2, set_port2, store_u16, load_u16)
    ];
}

impl YamlIOable for msgcont::Data<[u32;5]>{
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (ts2, set_ts2, store_u32, load_u32),
        (ts1trigger, set_ts1trigger, store_u8, load_u8),
        (ts1pps, set_ts1pps, store_u8, load_u8),
        (sss, set_sss, store_u16, load_u16),
        (event_count, set_event_count, store_u32, load_u32),
        (trig_pattern, set_trig_pattern, store_u8, load_u8)
    ];
}

impl YamlIOable for msgcont::Slc<[u32;16]>{
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (vpower1, set_vpower1, store_u16, load_u16),
        (vpower2, set_vpower2, store_u16, load_u16),
        (vpower3, set_vpower3, store_u16, load_u16),
        (vpower4, set_vpower4, store_u16, load_u16),
        (vpower5, set_vpower5, store_u16, load_u16),
        (vpower6, set_vpower6, store_u16, load_u16),
        (th1m, set_th1m, store_u16, load_u16),
        (th1p, set_th1p, store_u16, load_u16),
        (th2m, set_th2m, store_u16, load_u16),
        (th2p, set_th2p, store_u16, load_u16),
        (th3m, set_th3m, store_u16, load_u16),
        (th3p, set_th3p, store_u16, load_u16),
        (temp, set_temp, store_u16, load_u16),

        (total_trig_rate, set_total_rate, store_u32, load_u32),
        (ch1p_trig_rate, set_ch1p_trig_rate, store_u32, load_u32),
        (ch2p_trig_rate, set_ch2p_trig_rate, store_u32, load_u32),
        (ch3p_trig_rate, set_ch3p_trig_rate, store_u32, load_u32),
        (ch1m_trig_rate, set_ch1m_trig_rate, store_u32, load_u32),
        (ch2m_trig_rate, set_ch2m_trig_rate, store_u32, load_u32),
        (ch3m_trig_rate, set_ch3m_trig_rate, store_u32, load_u32),
        (max_coarse, set_max_coarse, store_u32, load_u32)
    ];
}

impl YamlIOable for msgcont::RdIntReg<[u32;13]>{
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (board_mac, set_board_mac, store_u32, load_u32),
        (board_ip, set_board_ip, store_u32, load_u32),
        (dst_mac1, set_dst_mac1, store_u64, load_u64),
        (dst_ip1,set_dst_ip1, store_u32, load_u32),
        (dst_mac2, set_dst_mac2, store_u64, load_u64),
        (dst_ip2,set_dst_ip2, store_u32, load_u32),
        (port1, set_port1, store_u16, load_u16),
        (port2, set_port2, store_u16, load_u16),
        (serial, set_serial, store_u64, load_u64)
    ];
}

impl YamlIOable for msgcont::Ack<[u32;2]>{
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (msg_ack, set_msg_ack, store_u16, load_u16)
    ];
}



impl YamlIOable for msg::TrendMsg{
    fn from_yaml(cfg:&Value)->Self{
        let msg_type=load_str(cfg, "msg_type").unwrap();
        match msg_type.as_str(){
            "DAQ"=>msg::TrendMsg::Daq{content:YamlIOable::from_yaml(cfg)},
            "TRIG"=>msg::TrendMsg::Trig{content:YamlIOable::from_yaml(cfg)},
            "SLCREQ"=>msg::TrendMsg::SlcReq,
            "GPS"=>msg::TrendMsg::Gps{content:YamlIOable::from_yaml(cfg), payload:load_vec_u8(cfg, "data").unwrap()},
            "ADC"=>msg::TrendMsg::Adc{content:YamlIOable::from_yaml(cfg)},
            "INTREG"=>msg::TrendMsg::IntReg{content:YamlIOable::from_yaml(cfg)},
            "DATA"=>msg::TrendMsg::Data{content:YamlIOable::from_yaml(cfg), payload:load_vec_u16(cfg, "data").unwrap()},
            "SLC" => msg::TrendMsg::Slc{content:YamlIOable::from_yaml(cfg)},
            "RDINTREG"=>msg::TrendMsg::RdIntReg{content:YamlIOable::from_yaml(cfg)},
            "ACK"=>msg::TrendMsg::Ack{content:YamlIOable::from_yaml(cfg)},
            _=>panic!()
        }
    }   

    fn to_yaml(&self)->Value{
        let mut result=
        match self{
            &msg::TrendMsg::Daq{ref content}=>content.to_yaml(),
            &msg::TrendMsg::Trig{ref content}=>content.to_yaml(),
            &msg::TrendMsg::SlcReq{..}=>From::from(Mapping::new()),
            &msg::TrendMsg::Gps{ref content,ref payload}=>{
                let mut x=content.to_yaml();
                x["data"]=From::<Vec<u16>>::from(payload.iter().map(|&x|{x as u16}).collect());
                x
            },
            &msg::TrendMsg::Adc{ref content}=>content.to_yaml(),
            &msg::TrendMsg::IntReg{ref content,..}=>content.to_yaml(),
            &msg::TrendMsg::Data{ref content,ref payload}=>{
                let mut x=content.to_yaml();
                x["data"]=From::from(payload.clone());
                x
            },
            &msg::TrendMsg::Slc{ref content,..}=>content.to_yaml(),
            &msg::TrendMsg::RdIntReg{ref content,..}=>content.to_yaml(),
            &msg::TrendMsg::Ack{ref content,..}=>content.to_yaml(),
        };
        result["name"]=From::from(self.type_name());
        result
    }
}