use super::field_element::FieldElement;
use super::point::Point;
use num_bigint::BigInt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct S256Field {
    num: usize,
    prime: BigInt,
}
pub struct S256Point {
    num: usize,
    prime: BigInt,
}

impl S256Field {
    fn new(num: usize) -> S256Field {
        let p: BigInt =
            (BigInt::from(2u32)).pow(256) - (BigInt::from(2u32)).pow(32) - BigInt::from(977);
        S256Field { num, prime: p }
    }
}
