use super::super::msg_def::msg;
use super::super::msg_def::msgcont;

use serde_yaml::{Mapping, Value};
use std::default::Default;

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

pub fn load_vec_to_u32(data: &Value, k: &str) -> Option<u32> {
    data[k].as_sequence().map(|x| {
        let mut result: u32 = 0;
        for (i, v) in x.iter().rev().enumerate() {
            result += (v.as_u64().unwrap() << (i * 8)) as u32;
        }
        result
    })
}

pub fn load_str(data: &Value, k: &str) -> Option<String> {
    data[k].as_str().map(|ref x| x.to_string())
}

pub fn str2u64(s: &str) -> Option<u64> {
    let r = if s.len() <= 2 {
        let a = u64::from_str_radix(s, 10);
        a
    } else if &s[0..2] == "0b" || &s[0..2] == "0B" {
        u64::from_str_radix(&s[2..], 2)
    } else if &s[0..2] == "0x" || &s[0..2] == "0X" {
        u64::from_str_radix(&s[2..], 16)
    } else {
        return None;
    };
    match r {
        Ok(x) => Some(x),
        _ => None,
    }
}

pub fn load_u64(data: &Value, k: &str) -> Option<u64> {
    if let Some(s) = data[k].as_str() {
        str2u64(s)
    } else {
        data[k].as_u64()
    }
}

pub fn load_u32(data: &Value, k: &str) -> Option<u32> {
    if let Some(s) = data[k].as_str() {
        str2u64(s).map(|x| x as u32)
    } else {
        data[k].as_u64().map(|x| x as u32)
    }
}

pub fn load_u16(data: &Value, k: &str) -> Option<u16> {
    if let Some(s) = data[k].as_str() {
        str2u64(s).map(|x| x as u16)
    } else {
        data[k].as_u64().map(|x| x as u16)
    }
}

pub fn load_u8(data: &Value, k: &str) -> Option<u8> {
    if let Some(s) = data[k].as_str() {
        str2u64(s).map(|x| x as u8)
    } else {
        data[k].as_u64().map(|x| x as u8)
    }
}

pub fn load_vpower1456(data: &Value, k: &str) -> Option<u16> {
    //encode
    if data[k].is_u64() {
        load_u16(data, k)
    } else if data[k].is_f64() {
        let vp = data[k].as_f64().unwrap();
        Some((vp / 5.0 * 2.0 / 24.0 * ((1_u16 << 12) as f64)) as u16)
    } else {
        None
    }
}

pub fn load_vpower23(data: &Value, k: &str) -> Option<u16> {
    //encode
    if data[k].is_u64() {
        load_u16(data, k)
    } else if data[k].is_f64() {
        let vp = data[k].as_f64().unwrap();
        Some((vp / 5.0 * 2.2 / 6.9 * ((1_u16 << 12) as f64)) as u16)
    } else {
        None
    }
}

pub fn load_th(data: &Value, k: &str) -> Option<u16> {
    if data[k].is_u64() {
        load_u16(data, k)
    } else if data[k].is_f64() {
        let th = data[k].as_f64().unwrap();
        Some((th / 0.5) as u16)
    } else {
        None
    }
}

pub fn load_temp(data: &Value, k: &str) -> Option<u16> {
    if data[k].is_u64() {
        load_u16(data, k)
    } else if data[k].is_f64() {
        let temp = data[k].as_f64().unwrap();
        let sign: u16 = if temp < 0. { 0b1000000000000 } else { 0 };
        let mut x12 = (temp.abs() / 0.0625) as u16 & 0b111111111111;
        if sign != 0 {
            x12 = !x12;
        }
        Some(sign + x12)
    } else {
        None
    }
}

pub fn store_u64(data: &mut Value, k: &str, v: u64) {
    data[k] = From::from(v);
}

pub fn store_u32(data: &mut Value, k: &str, v: u32) {
    data[k] = From::from(v);
}

pub fn store_u16(data: &mut Value, k: &str, v: u16) {
    data[k] = From::from(v);
}

pub fn store_u8(data: &mut Value, k: &str, v: u8) {
    data[k] = From::from(v);
}

pub fn store_vpower1456(data: &mut Value, k: &str, v: u16) {
    data[k] = From::from(v as f64 / ((1_u16 << 12) as f64) * 24.0 / 2.0 * 5.0);
}

pub fn store_u32_to_vec(data: &mut Value, k: &str, v: u32) {
    let mut result = Vec::<u8>::new();
    for i in 0..4 {
        result.push(((v >> ((3 - i) * 8)) & 0xff) as u8);
    }
    data[k] = From::from(result);
}

pub fn store_vpower23(data: &mut Value, k: &str, v: u16) {
    data[k] = From::from(v as f64 / ((1_u16 << 12) as f64) * 6.9 / 2.2 * 5.0);
}

pub fn store_th(data: &mut Value, k: &str, v: u16) {
    data[k] = From::from(v as f64 * 0.5);
}

pub fn store_temp(data: &mut Value, k: &str, v: u16) {
    let sign = v & 0b1000000000000;
    let mut x12 = v & 0b111111111111;
    if sign != 0 {
        x12 = (!x12) & 0b111111111111;
    }
    data[k] = From::from(if sign != 0 {
        -(x12 as f64) * 0.0625
    } else {
        x12 as f64 * 0.0625
    })
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

pub trait YamlIOable {
    fn from_yaml(cfg: &Value) -> Self;
    fn to_yaml(&self) -> Value;
}

impl YamlIOable for msgcont::Daq {
    yaml_io![
        (daq_on, set_daq_on, store_u8, load_u8),
        (ant_on, set_ant_on, store_u8, load_u8),
        (rd_wr_plus, set_rd_wr_plus, store_u8, load_u8),
        (en_osc, set_en_osc, store_u8, load_u8),
        (cntrl_adc, set_cntrl_adc, store_u8, load_u8),
        (offst, set_offst, store_u16, load_u16),
        (enable_pd, set_enable_pd, store_u8, load_u8),
        (dis_lna, set_dis_lna, store_u8, load_u8),
        (le, set_le, store_u8, load_u8),
        (att1, set_att1, store_u8, load_u8),
        (att2, set_att2, store_u8, load_u8)
    ];
}

impl YamlIOable for msgcont::Trig {
    yaml_io![
        (st, set_st, store_u8, load_u8),
        (trg_en, set_trg_en, store_u8, load_u8),
        (cntrl_dac, set_cntrl_dac, store_u16, load_u16),
        (th1m, set_th1m, store_th, load_th),
        (th1p, set_th1p, store_th, load_th),
        (th2m, set_th2m, store_th, load_th),
        (th2p, set_th2p, store_th, load_th),
        (th3m, set_th3m, store_th, load_th),
        (th3p, set_th3p, store_th, load_th)
    ];
}

impl YamlIOable for msgcont::Gps {
    yaml_io![
        (rwb, set_rwb, store_u8, load_u8),
        (addr, set_addr, store_u8, load_u8),
        (wrd, set_wrd, store_u8, load_u8)
    ];
}

impl YamlIOable for msgcont::Adc {
    yaml_io![
        (reg_func, set_reg_func, store_u16, load_u16),
        (addr, set_addr, store_u8, load_u8)
    ];
}

impl YamlIOable for msgcont::IntReg {
    yaml_io![
        (write, set_write, store_u8, load_u8),
        (board_mac, set_board_mac, store_u32, load_u32),
        (board_ip, set_board_ip, store_u32_to_vec, load_vec_to_u32),
        (srv_mac1, set_srv_mac1, store_u64, load_u64),
        (srv_ip1, set_srv_ip1, store_u32_to_vec, load_vec_to_u32),
        (srv_mac2, set_srv_mac2, store_u64, load_u64),
        (srv_ip2, set_srv_ip2, store_u32_to_vec, load_vec_to_u32),
        (port1, set_port1, store_u16, load_u16),
        (port2, set_port2, store_u16, load_u16)
    ];
}

impl YamlIOable for msgcont::Data {
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

impl YamlIOable for msgcont::Slc {
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (vpower1, set_vpower1, store_vpower1456, load_vpower1456),
        (vpower2, set_vpower2, store_vpower23, load_vpower23),
        (vpower3, set_vpower3, store_vpower23, load_vpower23),
        (vpower4, set_vpower4, store_vpower1456, load_vpower1456),
        (vpower5, set_vpower5, store_vpower1456, load_vpower1456),
        (vpower6, set_vpower6, store_vpower1456, load_vpower1456),
        (th1m, set_th1m, store_th, load_th),
        (th1p, set_th1p, store_th, load_th),
        (th2m, set_th2m, store_th, load_th),
        (th2p, set_th2p, store_th, load_th),
        (th3m, set_th3m, store_th, load_th),
        (th3p, set_th3p, store_th, load_th),
        (temp, set_temp, store_temp, load_temp),
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

impl YamlIOable for msgcont::RdIntReg {
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (board_mac, set_board_mac, store_u32, load_u32),
        (board_ip, set_board_ip, store_u32, load_u32),
        (srv_mac1, set_srv_mac1, store_u64, load_u64),
        (srv_ip1, set_srv_ip1, store_u32, load_u32),
        (srv_mac2, set_srv_mac2, store_u64, load_u64),
        (srv_ip2, set_srv_ip2, store_u32, load_u32),
        (port1, set_port1, store_u16, load_u16),
        (port2, set_port2, store_u16, load_u16),
        (serial, set_serial, store_u64, load_u64)
    ];
}

impl YamlIOable for msgcont::Ack {
    yaml_io![
        (ip, set_ip, store_u32, load_u32),
        (msg_ack, set_msg_ack, store_u16, load_u16)
    ];
}

impl YamlIOable for msg::TrendMsg {
    fn from_yaml(cfg: &Value) -> Self {
        let msg_type = load_str(cfg, "msg_type").expect("missing msg_type key");
        match msg_type.as_str() {
            "DAQ" => msg::TrendMsg::Daq {
                content: YamlIOable::from_yaml(cfg),
            },
            "TRIG" => msg::TrendMsg::Trig {
                content: YamlIOable::from_yaml(cfg),
            },
            "SLCREQ" => msg::TrendMsg::SlcReq,
            "GPS" => msg::TrendMsg::Gps {
                content: YamlIOable::from_yaml(cfg),
                payload: load_vec_u8(cfg, "data").map_or_else(
                    || {
                        eprintln!("Warning: payload not found, use []");
                        vec![]
                    },
                    |x| x,
                ),
            },
            "ADC" => msg::TrendMsg::Adc {
                content: YamlIOable::from_yaml(cfg),
            },
            "INTREG" => msg::TrendMsg::IntReg {
                content: YamlIOable::from_yaml(cfg),
            },
            "DATA" => msg::TrendMsg::Data {
                content: YamlIOable::from_yaml(cfg),
                payload: load_vec_u16(cfg, "data").map_or_else(
                    || {
                        eprintln!("Warning: payload not found, use []");
                        vec![]
                    },
                    |x| x,
                ),
            },
            "SLC" => msg::TrendMsg::Slc {
                content: YamlIOable::from_yaml(cfg),
            },
            "RDINTREG" => msg::TrendMsg::RdIntReg {
                content: YamlIOable::from_yaml(cfg),
            },
            "ACK" => msg::TrendMsg::Ack {
                content: YamlIOable::from_yaml(cfg),
            },
            _ => panic!(),
        }
    }

    fn to_yaml(&self) -> Value {
        let mut result = match self {
            &msg::TrendMsg::Daq { ref content } => content.to_yaml(),
            &msg::TrendMsg::Trig { ref content } => content.to_yaml(),
            &msg::TrendMsg::SlcReq { .. } => From::from(Mapping::new()),
            &msg::TrendMsg::Gps {
                ref content,
                ref payload,
            } => {
                let mut x = content.to_yaml();
                x["data"] = From::<Vec<u16>>::from(payload.iter().map(|&x| x as u16).collect());
                x
            }
            &msg::TrendMsg::Adc { ref content } => content.to_yaml(),
            &msg::TrendMsg::IntReg { ref content, .. } => content.to_yaml(),
            &msg::TrendMsg::Data {
                ref content,
                ref payload,
            } => {
                let mut x = content.to_yaml();
                x["data"] = From::from(payload.clone());
                x
            }
            &msg::TrendMsg::Slc { ref content, .. } => content.to_yaml(),
            &msg::TrendMsg::RdIntReg { ref content, .. } => content.to_yaml(),
            &msg::TrendMsg::Ack { ref content, .. } => content.to_yaml(),
        };
        result["msg_type"] = From::from(self.type_name());
        result
    }
}
