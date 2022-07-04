use impl_ops::*;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct S256Field {
    pub num: BigInt,
    pub prime: BigInt,
}

impl S256Field {
    pub fn new(num: BigInt) -> Self {
        let p = (BigInt::from(2u32)).pow(256) - (BigInt::from(2u32)).pow(32) - BigInt::from(977);
        Self {
            num: num % &p,
            prime: p,
        }
    }

    pub fn pow(self, exponent: BigInt) -> Self {
        let mut e = (BigInt::from(exponent) + &self.prime - 1) % (&self.prime - BigInt::from(1u8));
        let mut current = S256Field::new(BigInt::one());
        let mut coef = self.clone();
        while e > BigInt::zero() {
            if &e % BigInt::from(2u8) == One::one() {
                current = &current * &coef;
            }
            coef = &coef * &coef;
            e /= 2;
        }
        return current;
    }

    pub fn rmul(self, r: BigInt) -> Self {
        Self::new(self.num * r)
    }

    pub fn sqrt(self) -> Self {
        let p = self.prime.clone();
        return self.rmul((p + 1) / 4);
    }
}

impl_ops::impl_op_ex!(+ |a: &S256Field, b: &S256Field| -> S256Field {
    if &a.prime != &b.prime {
        panic!("can't add")
    };

    let num = (&a.num + &b.num) % &a.prime;
    return S256Field::new(num);
});

impl_ops::impl_op_ex!(-|a: &S256Field, b: &S256Field| -> S256Field {
    if &a.prime != &b.prime {
        panic!("can't sub")
    };

    let prime = &a.prime;
    let num = (&a.num - &b.num + prime) % prime;
    return S256Field::new(num);
});

impl_ops::impl_op_ex!(*|a: &S256Field, b: &S256Field| -> S256Field {
    if &a.prime != &b.prime {
        panic!("can't mul")
    };

    let num = (&a.num * &b.num) % &a.prime;
    return S256Field::new(num);
});

impl_ops::impl_op_ex!(/ |a: &S256Field, b: &S256Field| -> S256Field {
    if &a.prime != &b.prime {
        panic!("can't div")
    }
    if &b.num == &BigInt::zero() {
        panic!("can't div by zero");

    }

    let prime = &a.prime;
    // prime must be more than 2 because this means primary number
    let b_inverse = b.clone().pow((prime - BigInt::from(2u8)).try_into().unwrap());
    let num = (&a.num * b_inverse.num) % prime;
    return S256Field::new(num);
});
