use crate::util::reader::read;
use crate::util::varint;
use std::io::Read;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tx {
    version: u32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    locktime: Vec<u8>,
    testnet: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TxIn {
    prev_tx: Vec<u8>,
    prev_index: u32,
    script_sig: Vec<u8>,
    sequence: u32,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TxOut {
    amount: u64,
    script_pubkey: Vec<u8>,
}

impl Tx {
    fn parse<R>(reader: R, testnet: bool) -> Self
    where
        R: Read,
    {
        let buf = read(reader, 4);
        let version = u32::from_le_bytes(buf.try_into().unwrap());
        // parse Input portion
        let num_inputs = varint::read_varint(reader);
        let inputs: Vec<TxIn> = num_inputs.iter().map(|_| TxIn::parse(reader)).collect();
        // parse Output portion
        let num_outputs = varint::read_varint(reader);
        let outputs: Vec<TxOut> = num_inputs.iter().map(|_| TxOut::parse(reader)).collect();

        Self {
            version,
            tx_ins: inputs,
            tx_outs: outputs,
            locktime: None,
            testnet,
        }
    }
    fn serialize(self: &Self) -> Vec<u8> {}
}

impl TxIn {
    // Takes a byte stream and parses the tx_input at the start. Returns a TxIn struct.
    fn parse<R>(reader: R) -> Self
    where
        R: Read,
    {
        let prev_tx = read(reader, 32);
        let prev_index = u32::from_le_bytes(read(reader, 4).try_into().unwrap());
        let script_sig = Script::parse(reader);
        let sequence = u32::from_le_bytes(read(reader, 4).try_into().unwrap());
        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        }
    }
    fn serialize(self: &Self) -> Vec<u8> {}
}

impl TxOut {
    // Takes a byte stream and parses the tx_output at the start.  Returns a TxOut struct.
    fn parse<R>(reader: R) -> Self
    where
        R: Read,
    {
        let amount = u64::from_le_bytes(read(reader, 8).try_into().unwrap());
        let script_pubkey = Script::parse(reader);
        Self {
            amount,
            script_pubkey,
        }
    }
    fn serialize(self: &Self) -> Vec<u8> {}
}
