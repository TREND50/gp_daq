use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::hash::Hash;

pub struct TsCal<ID_T>
where ID_T:std::cmp::Eq+Hash
{
    shifts:HashMap<ID_T, f64>,
}

const UPDATE_COEFF:f64=0.1;

impl<ID_T> TsCal<ID_T>
where ID_T:std::cmp::Eq+Hash+Copy
{
    pub fn new()->TsCal<ID_T>{
        TsCal{shifts:HashMap::new()}
    }

    pub fn update(&mut self, ip:ID_T, sys_ts:f64, board_ts:f64)->i64{
        let diff=sys_ts-board_ts;
        let shift=match self.shifts.entry(ip){
            Occupied(mut x)=>{
                let old=*x.get();
                let y=(old*(1.0-UPDATE_COEFF)+UPDATE_COEFF*diff);
                if(old-y).abs()>2.0{
                    eprintln!("WARNING, ts jump");
                }
                *x.get_mut()=y;
                y
            }
            Vacant(x)=>{
                x.insert(diff);
                diff
            }
        };

        return diff.round() as i64;
    }
}