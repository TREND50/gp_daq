use chrono::offset::Utc;
use chrono::DateTime;
use serde_yaml::Value;

pub fn add_source_info(v: &mut Value, now: &DateTime<Utc>, ip: &[i64]) {
    v["received_timestamp"] = From::from(vec![
        now.timestamp(),
        i64::from(now.timestamp_subsec_nanos()),
    ]);
    v["received_timestamp_str"] = From::from(now.to_string());
    v["source_ip"] = From::from(ip.to_owned());
}
