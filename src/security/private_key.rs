use super::signature::Signature;
use crate::ecc::s256_point::S256Point;
use crate::util::base58::encode_base58_checksum;
use bs58::encode;
use num_bigint::{BigInt, RandBigInt};
use num_traits::Zero;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrivateKey {
    secret: BigInt,
    pub point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: BigInt) -> Self {
        let g = S256Point::new_g();
        let p = g.rmul(secret.clone());
        Self { secret, point: p }
    }

    pub fn hex(&self) -> String {
        return format!("{:0>64}", self.secret.to_string());
    }

    pub fn sign(self, z: &BigInt) -> Signature {
        let g = S256Point::new_g();
        let n = g.n.clone();
        let mut rng = rand::thread_rng();
        let k = rng.gen_bigint_range(&BigInt::zero(), &n);
        let r = g.rmul(k.clone()).x.unwrap().num;
        let k_inv = k.modpow(&(n.clone() - BigInt::from(2u8)), &n);
        let mut s = (z + &r * self.secret) * k_inv % &n;
        if s > &n / BigInt::from(2u8) {
            s = &n - s;
        }
        return Signature::new(r, s);
    }

    pub fn wif(self, compressed: bool, testnet: bool) -> String {
        let (_, mut secret_byte) = self.secret.to_bytes_be();
        let diff = 32 - secret_byte.len();
        for _ in 0..diff {
            secret_byte.insert(0, 0x0);
        }

        if testnet {
            secret_byte.insert(0, 0xef);
        } else {
            secret_byte.insert(0, 0x80);
        };
        if compressed {
            secret_byte.push(0x01);
        }

        encode_base58_checksum(&secret_byte)
    }

    // !TODO: I don't figure out whether this function is needed or not
    // fn deterministic_k(self, z: BigInt) {
    //     let k = BigInt::parse_bytes(b"00", 16).unwrap() * BigInt::from(32u8);
    //     let v = BigInt::parse_bytes(b"01", 16).unwrap() * BigInt::from(32u8);
    //     let n = N::new().value;
    //     let mut z_brw = z;
    //     if z_brw > n {
    //         z_brw -= n;
    //     }
    //     let z_bytes = z_brw.to_bytes_be();
    //     let secret_bytes = self.secret.to_bytes_be();
    // }
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

    #[test]
    fn wif_test1() {
        let prv = PrivateKey::new(BigInt::from(5003));
        assert_eq!(
            prv.wif(true, true),
            "cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK"
        );
    }
    #[test]
    fn wif_test2() {
        let prv = PrivateKey::new(BigInt::from(2021).pow(5));
        assert_eq!(
            prv.wif(false, true),
            "91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic"
        );
    }
    #[test]
    fn wif_test3() {
        let key = b"54321deadbeef";
        let prv = PrivateKey::new(BigInt::parse_bytes(key, 16).unwrap());
        assert_eq!(
            prv.wif(true, false),
            "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a"
        );
    }
}
