use bitcoin_hashes::{sha256, Hash};
pub fn hash256(byte: &[u8]) -> Vec<u8> {
    sha256::Hash::hash(&sha256::Hash::hash(byte).to_vec()).to_vec()
}
