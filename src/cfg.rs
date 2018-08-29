use super::msg;
use super::msgcont;

use serde_yaml::{Value};

pub fn load_vec_u64(data: &Value, k: &str) -> Option<Vec<u64>> {
    data[k]
        .as_sequence()
        .map(|x| x.iter().map(|ref x| x.as_u64().unwrap()).collect())
}

pub fn load_u64(data: &Value, k: &str) -> Option<u64> {
    data[k].as_u64()
}

