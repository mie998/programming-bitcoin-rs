#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tx {
    version: Vec<u8>,
    tx_ins: Vec<u8>,
    tx_outs: Vec<u8>,
    locktime: Vec<u8>,
    testnet: bool,
}

impl Tx {
    fn parse() -> Self {}
}
