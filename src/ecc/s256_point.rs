use crate::security::signature::Signature;

use super::field_element::FieldElement;
use super::s256_field::S256Field;
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

    pub fn sec_str(self, compress: bool) -> String {
        // !TODO: 以下の実装だと符号情報 Sign が落ちる。でも、符号情報を扱うと bit数が増えるのでどうしたものか、、、
        // !TODO: 32byte に満たない数の場合は左に zero padding しているが、いいのだろうか。
        if compress {
            let (_, vec_x) = self.x.unwrap().num.to_radix_be(16);
            let x_coordinate = vec_x
                .iter()
                .map(|x| char::from_digit(*x as u32, 16).unwrap())
                .collect::<String>();
            let x_coordinate_padding = format!("{:0>32}", x_coordinate);

            let marker = if self.y.unwrap().num % BigInt::from(2u8) == BigInt::from(0u8) {
                String::from("02")
            } else {
                String::from("03")
            };

            marker + &x_coordinate_padding
        } else {
            let (_, vec_x) = self.x.unwrap().num.to_radix_be(16);
            let x_coordinate = vec_x
                .iter()
                .map(|x| char::from_digit(*x as u32, 16).unwrap())
                .collect::<String>();
            // 32 byte (string.len = 64) になるまで 左に zero-padding
            let x_coordinate_padding = format!("{:0>64}", x_coordinate);

            let (_, vec_y) = self.y.unwrap().num.to_radix_be(16);
            let y_coordinate = vec_y
                .iter()
                .map(|y| char::from_digit(*y as u32, 16).unwrap())
                .collect::<String>();
            // 32 byte (string.len = 64) になるまで 左に zero-padding
            let y_coordinate_padding = format!("{:0>64}", y_coordinate);

            let marker = String::from("04");

            marker + &x_coordinate_padding + &y_coordinate_padding
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

    #[test]
    fn sec_str_not_compressed1() {
        let prv = PrivateKey::new(BigInt::from(5000u32));
        let s = prv.point.sec_str(false);
        assert_eq!(
            s,
            "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"
        );
    }

    #[test]
    fn sec_str_not_compressed2() {
        let prv = PrivateKey::new(BigInt::from(2018).pow(5));
        let s = prv.point.sec_str(false);
        assert_eq!(
            s,
            "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06"
        );
    }

    #[test]
    fn sec_str_not_compressed3() {
        let key = b"deadbeef12345";
        let prv = PrivateKey::new(BigInt::parse_bytes(key, 16).unwrap());
        let s = prv.point.sec_str(false);
        assert_eq!(
            s,
            "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"
        );
    }
}
