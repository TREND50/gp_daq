use tokio::codec::Decoder;
use super::super::msg_def::msg::TrendMsg;
use bytes::BytesMut;

use std;

pub struct MsgDecoder{

}

impl Decoder for MsgDecoder{
    type Item=TrendMsg;
    type Error=std::io::Error;


    fn decode(&mut self, src:&mut BytesMut)->Result<Option<Self::Item>, Self::Error>{
        let mut buff=vec![];
        buff.extend_from_slice(&src.take());
        Ok(TrendMsg::from_byte_vec(buff))
    }
}