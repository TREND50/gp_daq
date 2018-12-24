use std::error::Error;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub enum NetErr {
    NoAck,
    NotAck,
    AckTypeMismatch,
}

impl Display for NetErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NetErr::NoAck => write!(f, "NoAck"),
            NetErr::NotAck => write!(f, "Something received, but not Ack"),
            NetErr::AckTypeMismatch => write!(f, "Ack received, but type code mismatch"),
        }
    }
}

impl Error for NetErr {}
