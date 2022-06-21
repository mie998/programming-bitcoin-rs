use super::signature::Signature;
use crate::ecc::point::{Point, N};
use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrivateKey {
    secret: BigInt,
    point: Point,
}

impl PrivateKey {
    pub fn new(secret: BigInt) -> Self {
        let g = Point::new_g();
        let p = g.rmul_s256(secret.clone());
        Self { secret, point: p }
    }

    fn hex(&self) -> String {
        return format!("{:0>64}", self.secret.to_string());
    }

    fn sign(self, z: &BigInt) -> Signature {
        let n = N::new().value;
        let g = Point::new_g();
        let mut rng = rand::thread_rng();
        let k = rng.gen_bigint_range(&BigInt::zero(), &n);
        let r = g.rmul_s256(k.clone()).x.unwrap().num;
        let k_inv = k.modpow(&(n.clone() - BigInt::from(2u8)), &n);
        let mut s = (z + &r * self.secret) * k_inv % &n;
        if s > &n / BigInt::from(2u8) {
            s = n - s;
        }
        return Signature::new(r, s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_test() {
        let b = BigInt::from(8u8);
        let pk = PrivateKey::new(b);
        assert_eq!(
            pk.hex(),
            "0000000000000000000000000000000000000000000000000000000000000008"
        )
    }
}
