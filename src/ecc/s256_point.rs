use crate::security::signature::Signature;

use super::field_element::FieldElement;
use super::s256_field::S256Field;
use crate::util::{base58, hash160};
use impl_ops::*;
use num_bigint::{BigInt, Sign};
use num_traits::{One, Zero};
use std::ops::{self};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct S256Point {
    pub x: Option<S256Field>,
    pub y: Option<S256Field>,
    pub a: S256Field,
    pub b: S256Field,
    pub n: BigInt,
}

impl S256Point {
    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> S256Point {
        let a = S256Field::new(BigInt::zero());
        let b = S256Field::new(BigInt::from(7u8));
        let bytes_n = b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
        let n = BigInt::parse_bytes(bytes_n, 16).unwrap();
        match (&x, &y) {
            (None, _) | (_, None) => S256Point {
                x: None,
                y: None,
                a,
                b,
                n,
            },
            (Some(x), Some(y)) => {
                if y.clone().pow(BigInt::from(2u8))
                    != x.clone().pow(BigInt::from(3u8)) + &a * x + &b
                {
                    panic!("Points {:?} {:?} are not on the curve!", x, y);
                }
                S256Point {
                    x: Some(x.clone()),
                    y: Some(y.clone()),
                    a,
                    b,
                    n,
                }
            }
        }
    }

    pub fn new_u64(x: u64, y: u64) -> Self {
        let xf = S256Field::new(BigInt::from(x));
        let yf = S256Field::new(BigInt::from(y));
        Self::new(Some(xf), Some(yf))
    }

    pub fn new_g() -> Self {
        let bytes_x = b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let bytes_y = b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let x = S256Field::new(BigInt::parse_bytes(bytes_x, 16).unwrap());
        let y = S256Field::new(BigInt::parse_bytes(bytes_y, 16).unwrap());
        Self::new(Some(x), Some(y))
    }

    pub fn rmul(self, coefficient: BigInt) -> Self {
        let mut coef = coefficient.clone() % self.n.clone();
        let mut current = self.clone();
        let mut result = Self::new(None, None);

        while coef > BigInt::zero() {
            if &coef % BigInt::from(2u8) == BigInt::one() {
                result = &result + &current;
            };
            current = &current + &current;
            coef /= BigInt::from(2u8);
        }

        return result;
    }

    pub fn verify(self, z: BigInt, sig: Signature) -> bool {
        let n = self.n.clone();
        let s = FieldElement::new(sig.s, n.clone());
        let s_inv = FieldElement::pow(s, n - BigInt::from(2u8));
        let u = &s_inv.clone().rmul(z);
        let v = &s_inv.clone().rmul(sig.r.clone());
        let total = Self::new_g().rmul(u.num.clone()) + self.rmul(v.num.clone());

        return total.x.unwrap().num == sig.r;
    }

    pub fn sec(self, compress: bool) -> Vec<u8> {
        // !TODO: 以下の実装だと符号情報 Sign が落ちる。でも、符号情報を扱うと bit数が増えるのでどうしたものか、、、
        if compress {
            let (_, vec_x) = self.x.unwrap().num.to_bytes_be();
            // 32 byte (vec.len = 32) になるまで 左に zero-padding
            let diff = 32 - vec_x.len();
            let mut vec_x_with_padding = std::iter::repeat(0x0).take(diff).collect::<Vec<u8>>();
            vec_x_with_padding.extend(vec_x);

            let marker = if self.y.unwrap().num % BigInt::from(2u8) == BigInt::from(0u8) {
                0x02
            } else {
                0x03
            };

            let mut result = Vec::new();
            result.push(marker);
            result.extend(vec_x_with_padding);

            result
        } else {
            let (_, vec_x) = self.x.unwrap().num.to_bytes_be();
            let diff_x = 32 - vec_x.len();
            // 32 byte (vec.len = 32) になるまで 左に zero-padding
            let mut vec_x_with_padding = std::iter::repeat(0x0).take(diff_x).collect::<Vec<u8>>();
            vec_x_with_padding.extend(vec_x);

            let (_, vec_y) = self.y.unwrap().num.to_bytes_be();
            let diff_y = 32 - vec_y.len();
            // 32 byte (vec.len = 32) になるまで 左に zero-padding
            let mut vec_y_with_padding = std::iter::repeat(0x0).take(diff_y).collect::<Vec<u8>>();
            vec_y_with_padding.extend(vec_y);

            let mut result = Vec::new();
            result.push(0x04);
            result.extend(vec_x_with_padding);
            result.extend(vec_y_with_padding);

            result
        }
    }

    pub fn parse(self, sec_bin: Vec<u8>) -> Self {
        // return Point from SEC binary (not 16 bigit)
        if sec_bin[0] == 4 {
            let x = BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..33]);
            let y = BigInt::from_bytes_be(Sign::Plus, &sec_bin[33..65]);
            return Self::new(Some(S256Field::new(x)), Some(S256Field::new(y)));
        } else {
            let is_even = sec_bin[0] == 2;
            let x = S256Field::new(BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..]));
            // get the answer of formular y^2 = x^3 + 7
            let alpha = &x * &x * &x + S256Field::new(self.b.num);
            let beta = alpha.sqrt();
            match (&beta.num % BigInt::from(2u8) == BigInt::from(0u8), is_even) {
                (true, true) | (false, false) => {
                    Self::new(Some(S256Field::new(x.num)), Some(S256Field::new(beta.num)))
                }
                (true, false) | (false, true) => Self::new(
                    Some(S256Field::new(x.num)),
                    Some(S256Field::new(beta.prime - beta.num)),
                ),
            }
        }
    }

    pub fn hash160(self, compressed: bool) -> Vec<u8> {
        hash160::hash160(&self.sec(compressed))
    }

    pub fn address(self, compressed: bool, testnet: bool) -> String {
        let mut h160 = self.hash160(compressed);
        let prefix = if testnet { 0x6f } else { 0x00 };
        h160.insert(0, prefix);
        base58::encode_base58_checksum(&h160)
    }
}

impl_ops::impl_op_ex!(+ |p1: &S256Point, p2: &S256Point| -> S256Point{
        // case: S256Points are n qot on the same curve
        if p1.a != p2.a || p1.b != p2.b {
            panic!("S256Points {:?} {:?} are not on the same curve!", p1, p2);
        };

        // case: S256Points are on the same curve and meet a special condition
        if p1 == p2 {
            let (p1_x, p1_y) = (p1.x.as_ref().unwrap(), p1.y.as_ref().unwrap());
            if p1_x.clone() == S256Field::new(BigInt::zero()) * p1_y {
                return S256Point::new(None, None);
            } else {
                let s = (S256Field::new(BigInt::from(3u8)) * p1_x.clone().pow(BigInt::from(2u8)) + &p1.a) / (S256Field::new(BigInt::from(2u8)) * p1_y);
                let x = s.clone().pow(BigInt::from(2u8)) - S256Field::new(BigInt::from(2u8)) * p1_x;
                let y = s * (p1_x - x.clone()) - p1_y;
                return S256Point::new(Some(x), Some(y));
            }
        }

        // case: S256Points are on the same curve and on normal positions
        match (&p1.x, &p2.x) {
            (None, _) => p2.clone(),
            (_, None) => p1.clone(),
            (Some(p1_x), Some(p2_x)) => {
                if p1_x == p2_x {
                    S256Point::new(None, None)
                } else {
                    let (p1_y, p2_y) = (p1.y.as_ref().unwrap(), p2.y.as_ref().unwrap());
                    let s = (p2_y - p1_y) / (p2_x - p1_x);
                    let x = s.clone().pow(BigInt::from(2u8)) - p1_x - p2_x;
                    let y = s * (p1_x - x.clone()) - p1_y;
                    S256Point::new(Some(x), Some(y))
                }
            }
        }
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::private_key::PrivateKey;
    use crate::util::hex::hex;

    #[test]
    fn sec_not_compressed1() {
        let prv = PrivateKey::new(BigInt::from(5000u32));
        let v = prv.point.sec(false);
        assert_eq!(
            hex(&v),
            "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"
        );
    }

    #[test]
    fn sec_not_compressed2() {
        let prv = PrivateKey::new(BigInt::from(2018).pow(5));
        let v = prv.point.sec(false);
        assert_eq!(
            hex(&v),
            "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06"
        );
    }

    #[test]
    fn sec_not_compressed3() {
        let key = b"deadbeef12345";
        let prv = PrivateKey::new(BigInt::parse_bytes(key, 16).unwrap());
        let v = prv.point.sec(false);
        assert_eq!(
            hex(&v),
            "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"
        );
    }
    #[test]
    fn sec_compressed1() {
        let prv = PrivateKey::new(BigInt::from(5001u32));
        let v = prv.point.sec(true);
        assert_eq!(
            hex(&v),
            "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1"
        );
    }

    #[test]
    fn sec_compressed2() {
        let prv = PrivateKey::new(BigInt::from(2019).pow(5));
        let v = prv.point.sec(true);
        assert_eq!(
            hex(&v),
            "02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701"
        );
    }

    #[test]
    fn sec_compressed3() {
        let key = b"deadbeef54321";
        let prv = PrivateKey::new(BigInt::parse_bytes(key, 16).unwrap());
        let v = prv.point.sec(true);
        assert_eq!(
            hex(&v),
            "0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690"
        );
    }

    #[test]
    fn address1() {
        let prv = PrivateKey::new(BigInt::from(5002));
        assert_eq!(
            prv.point.address(false, true),
            "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA"
        );
    }
    #[test]
    fn address2() {
        let prv = PrivateKey::new(BigInt::from(2020).pow(5));
        assert_eq!(
            prv.point.address(true, true),
            "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH"
        );
    }
    #[test]
    fn address3() {
        let key = b"12345deadbeef";
        let prv = PrivateKey::new(BigInt::parse_bytes(key, 16).unwrap());
        assert_eq!(
            prv.point.address(true, false),
            "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1"
        );
    }
}
