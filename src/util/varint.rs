use tokio::io::{self, AsyncRead};

pub fn read_varint(s: tokio::io::ReadBuf) -> i64 {}

pub fn encode_varint(i: i64) -> Vec<u8> {}
