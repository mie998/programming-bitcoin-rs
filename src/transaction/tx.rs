use crate::util::reader::read;
use crate::util::varint::{self, encode_varint};
use std::io::Read;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tx {
    version: u32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    locktime: u32,
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

// !TODO: 5.5章, 練習問題5 をテストに追加する。
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
        // parse Locktime portion
        let locktime = u32::from_le_bytes(read(reader, 4).try_into().unwrap());

        Self {
            version,
            tx_ins: inputs,
            tx_outs: outputs,
            locktime,
            testnet,
        }
    }

    fn serialize(self: &Self) -> Vec<u8> {
        let mut result = u32::to_le_bytes(self.version).to_vec();
        result.extend(encode_varint(self.tx_ins.len()).unwrap());
        result.extend(
            self.tx_ins
                .iter()
                .flat_map(|tx_in| tx_in.serialize())
                .collect::<Vec<u8>>(),
        );
        result.extend(encode_varint(self.tx_outs.len()).unwrap());
        result.extend(
            self.tx_outs
                .iter()
                .flat_map(|tx_out| tx_out.serialize())
                .collect::<Vec<u8>>(),
        );
        result.extend(u32::to_le_bytes(self.locktime).to_vec());
        result
    }
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

    fn serialize(self: &Self) -> Vec<u8> {
        let mut result: Vec<u8> = self.prev_tx.into_iter().rev().collect();
        result.extend(u32::to_le_bytes(self.prev_index));
        result.extend(self.script_sig.serialize());
        result.extend(u32::to_le_bytes(self.sequence));
        result
    }
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

    fn serialize(self: &Self) -> Vec<u8> {
        let mut result = u64::to_le_bytes(self.amount).to_vec();
        result.extend(self.script_pubkey.serialize());
        result
    }
}
