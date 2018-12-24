#![allow(clippy::many_single_char_names)]
#![allow(clippy::identity_op)]
use super::super::msg_def::msg;
use chrono::offset::Utc;
use chrono::DateTime;
use std;
use std::io::Write;

fn decode_vpower1456(v: u16) -> f64 {
    f64::from(v) / f64::from(1_u16 << 12) * 24.0 / 2.0 * 5.0
}

fn decode_vpower23(v: u16) -> f64 {
    f64::from(v) / f64::from(1_u16 << 12) * 6.9 / 2.2 * 5.0
}

fn decode_th(v: u16) -> f64 {
    f64::from(v) * 0.5
}

fn decode_temp(v: u16) -> f64 {
    let sign = v & 0b1_0000_0000_0000;
    let mut x12 = v & 0b1111_1111_1111;
    if sign != 0 {
        x12 = (!x12) & 0b1111_1111_1111;
    }
    if sign != 0 {
        -f64::from(x12) * 0.0625
    } else {
        f64::from(x12) * 0.0625
    }
}

fn decode_ip(v: u32) -> [u8; 4] {
    let a = ((v & 0xff00_0000) >> 24) as u8;
    let b = ((v & 0xff_0000) >> 16) as u8;
    let c = ((v & 0xff00) >> 8) as u8;
    let d = ((v & 0xff) >> 0) as u8;
    [a, b, c, d]
}

impl msg::TrendMsg {
    #[deprecated(
        note = "the original cfg format will be unsupported soon, please switch to yaml asap"
    )]
    pub fn write_to_txt<T: Write>(
        &self,
        write: &mut T,
        date: &DateTime<Utc>,
    ) -> Result<(), std::io::Error> {
        match *self {
            msg::TrendMsg::Data {
                ref content,
                ref payload,
            } => {
                writeln!(write, "-----------------")?;
                writeln!(write, "{}", date.to_string())?;
                let ip = decode_ip(content.ip());
                writeln!(write, "IP: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])?;
                writeln!(write, "TS2: {}", content.ts2())?;
                writeln!(write, "TS1Trigger: {}", content.ts1trigger())?;
                writeln!(write, "TS1PPS: {}", content.ts1pps())?;
                writeln!(write, "SSS: {}", content.sss())?;
                writeln!(write, "EC: {}", content.event_count())?;
                writeln!(write, "TrigPattern: {}", content.trig_pattern())?;
                for i in payload {
                    write!(write, "{} ", i)?;
                }
                writeln!(write).unwrap();
            }
            msg::TrendMsg::Slc { ref content } => {
                writeln!(write, "-----------------")?;
                writeln!(write, "{}", date.to_string())?;
                let ip = decode_ip(content.ip());
                writeln!(write, "IP: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])?;
                writeln!(write, "VPower1: {}", decode_vpower1456(content.vpower1()))?;
                writeln!(write, "VPower2: {}", decode_vpower23(content.vpower2()))?;
                writeln!(write, "VPower3: {}", decode_vpower23(content.vpower3()))?;
                writeln!(write, "VPower4: {}", decode_vpower1456(content.vpower4()))?;
                writeln!(write, "VPower5: {}", decode_vpower1456(content.vpower5()))?;
                writeln!(write, "VPower6: {}", decode_vpower1456(content.vpower6()))?;
                writeln!(
                    write,
                    "[Th1-,Th1+]: {} {}",
                    decode_th(content.th1m()),
                    decode_th(content.th1p())
                )?;
                writeln!(
                    write,
                    "[Th2-,Th2+]: {} {}",
                    decode_th(content.th2m()),
                    decode_th(content.th2p())
                )?;
                writeln!(
                    write,
                    "[Th3-,Th3+]: {} {}",
                    decode_th(content.th3m()),
                    decode_th(content.th3p())
                )?;
                writeln!(write, "Temp: {}", decode_temp(content.temp()))?;
                writeln!(write, "TotalTrigRate: {}", content.total_trig_rate())?;
                writeln!(write, "Ch1+TrigRate: {}", content.ch1p_trig_rate())?;
                writeln!(write, "Ch2+TrigRate: {}", content.ch2p_trig_rate())?;
                writeln!(write, "Ch3+TrigRate: {}", content.ch3p_trig_rate())?;
                writeln!(write, "Ch1-TrigRate: {}", content.ch1m_trig_rate())?;
                writeln!(write, "Ch2-TrigRate: {}", content.ch2m_trig_rate())?;
                writeln!(write, "Ch3-TrigRate: {}", content.ch3m_trig_rate())?;
                writeln!(write, "MaxCoarse: {}", content.max_coarse())?;
            }
            _ => {}
        }
        Ok(())
    }
}
