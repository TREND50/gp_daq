#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Read, Write};
use std::fmt::Debug;
#[derive(Default)]
pub struct TsCal<IdT>
where
    IdT: std::cmp::Eq + Hash+Debug,
{
    shifts: HashMap<IdT, f64>,
    pub cnt: usize,
}

const UPDATE_COEFF: f64 = 0.01;

impl<IdT> TsCal<IdT>
where
    IdT: std::cmp::Eq + Hash + Copy + Debug,
{
    pub fn new() -> TsCal<IdT> {
        TsCal {
            shifts: HashMap::new(),
            cnt:0,
        }
    }

    pub fn update(&mut self, ip: IdT, sys_ts: f64, board_ts: f64) -> i64 {
        let diff = sys_ts - board_ts;
        self.cnt+=1;
        let shift = match self.shifts.entry(ip) {
            Occupied(mut x) => {
                let old = *x.get();
                let y = old * (1.0 - UPDATE_COEFF) + UPDATE_COEFF * diff;
                if (old.round() - y.round()).abs() as i32 >= 1 {
                    eprintln!("WARNING, ts jump");
                    eprintln!("{} {} {} {}", sys_ts as u64, board_ts as u64, y as u64, diff);
                    panic!();
                }
                if self.cnt%1000==0{
                    let mut tsc_file =
                        OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("tsdump.txt")
                            .expect("cannot open text file for data");

                    writeln!(&mut tsc_file, "{} {} {} {}", sys_ts as u64, board_ts as u64, y.round(), diff).unwrap();
                }

                *x.get_mut() = y;
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
