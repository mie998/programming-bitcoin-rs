#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Signature {
    r: BigInt,
    s: BigInt,
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Self {
        Self { r, s }
    }
}
