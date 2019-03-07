#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Read, Write};

#[derive(Default)]
pub struct TsCal<IdT>
where
    IdT: std::cmp::Eq + Hash,
{
    shifts: HashMap<IdT, f64>,
}

const UPDATE_COEFF: f64 = 0.01;

impl<IdT> TsCal<IdT>
where
    IdT: std::cmp::Eq + Hash + Copy,
{
    pub fn new() -> TsCal<IdT> {
        TsCal {
            shifts: HashMap::new(),
        }
    }

    pub fn update(&mut self, ip: IdT, sys_ts: f64, board_ts: f64) -> i64 {
        let diff = sys_ts - board_ts;
        let shift = match self.shifts.entry(ip) {
            Occupied(mut x) => {
                let old = *x.get();
                let y = old * (1.0 - UPDATE_COEFF) + UPDATE_COEFF * diff;
                if (old - y).abs() >= 1.0 {
                    eprintln!("WARNING, ts jump");
                }
                *x.get_mut() = y;
                eprintln!("{} {} {} {}", sys_ts, board_ts, diff, y);
                y
            }
            Vacant(x) => {
                x.insert(diff);
                diff
            }
        };

        /*
        let mut f = match OpenOptions::new().append(true).open("shift.txt") {
                        Ok(f) => f,
                        _ => File::create("shift.txt").unwrap(),
                    };
        writeln!(f, "{} {} {}", shift, sys_ts, board_ts);
        */
        shift.round() as i64
    }
}
