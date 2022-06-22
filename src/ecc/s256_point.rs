use crate::security::signature::Signature;

use super::field_element::FieldElement;
use super::s256_field::S256Field;
use impl_ops::*;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use std::ops::{self, BitAnd};

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

    // pub fn new_u64_s256(x: u64, y: u64) -> Self {
    //     let a = FE::new_s256(BigInt::zero());
    //     let b = FE::new_s256(BigInt::from(7u8));
    //     let xf = FE::new_s256(BigInt::from(x));
    //     let yf = FE::new_s256(BigInt::from(y));
    //     Self::new(Some(xf), Some(yf), a, b)
    // }

    pub fn new_g() -> Self {
        let bytes_x = b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let bytes_y = b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let x = S256Field::new(BigInt::parse_bytes(bytes_x, 16).unwrap());
        let y = S256Field::new(BigInt::parse_bytes(bytes_y, 16).unwrap());
        Self::new(Some(x), Some(y))
    }

    pub fn rmul(self, coefficient: BigInt) -> Self {
        let p = self.x.clone().unwrap().prime;
        if self.a.prime != p {
            panic!("This method shouldn't be used with this prime value.")
        }

        let n = self.n.clone();
        Self::rmul(self, coefficient % n)
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
                let p = &p1_x.prime;
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
