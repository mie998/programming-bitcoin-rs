use crate::ecc::point::Point;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrivateKey {
    secret: Point,
    point: Point,
}

impl PrivateKey {
    pub fn new(secret: Point) -> Self {
        let g = Point::new_g();
        let p = secret * g;
        Self { secret, point: p }
    }
}
