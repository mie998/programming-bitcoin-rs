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

    // !TODO: test 追加
    fn hex(self) {
        return format!("{{:0width$}}", self.secret, 64);
    }

    // !TODO: test 追加
    fn sign(self, z: BigInt) {
        let n = N::new();
        let k = BigInt::from(n - 1);
        let r = (k * G);
    }
}
