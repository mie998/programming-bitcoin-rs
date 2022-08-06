use bitcoin_hashes::{ripemd160, sha256, Hash};

#[allow(dead_code)]
pub fn hash160(byte: &[u8]) -> Vec<u8> {
    let sha256 = sha256::Hash::hash(byte);
    ripemd160::Hash::hash(&sha256).to_vec()
}
