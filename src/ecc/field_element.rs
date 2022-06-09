use impl_ops::*;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> Self {
        if prime < BigInt::from(2u8) {
            panic!("prime should be more than 2 or equal to 2")
        }

        Self {
            num: num % &prime,
            prime,
        }
    }

    pub fn pow(self, exponent: i32) -> Self {
        let mut e = (BigInt::from(exponent) + &self.prime - 1) % (&self.prime - BigInt::from(1u8));
        let mut current = FieldElement::new(BigInt::one(), self.prime.clone());
        let mut coef = self.clone();
        while e > BigInt::zero() {
            if &e % BigInt::from(2u8) == One::one() {
                current = current.clone() * coef.clone();
            }
            coef = coef.clone() * coef.clone();
            e /= 2;
        }
        return current;
    }
}

impl_ops::impl_op_ex!(+ |a: &FieldElement, b: &FieldElement| -> FieldElement {
    if &a.prime != &b.prime {
        panic!("can't add")
    };

    let num = (&a.num + &b.num) % &a.prime;
    return FieldElement::new(num, a.prime.clone());
});

impl_ops::impl_op_ex!(-|a: &FieldElement, b: &FieldElement| -> FieldElement {
    if &a.prime != &b.prime {
        panic!("can't sub")
    };

    let prime = &a.prime;
    let num = (&a.num - &b.num + prime) % prime;
    return FieldElement::new(num, a.prime.clone());
});

impl_ops::impl_op_ex!(*|a: &FieldElement, b: &FieldElement| -> FieldElement {
    if &a.prime != &b.prime {
        panic!("can't sub")
    };

    let num = (&a.num * &b.num) % &a.prime;
    return FieldElement::new(num, a.prime.clone());
});

impl_ops::impl_op_ex!(/|a: &FieldElement, b: &FieldElement| -> FieldElement {
    if &a.prime != &b.prime {
        panic!("can't sub")
    }
    if &b.num <= &BigInt::zero() {
        panic!("can't div by zero");

    }

    let prime = &a.prime;
    // prime must be more than 2 because this means primary number
    let b_inverse = b.clone().pow((prime - BigInt::from(2u8)).try_into().unwrap());
    let num = (&a.num * b_inverse.num) % prime;
    return FieldElement::new(num, a.prime.clone());
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow() {
        let f1 = FieldElement::new(BigInt::from(9i8), BigInt::from(13));
        let e = 3;
        let f2 = FieldElement::new(BigInt::from(1), BigInt::from(13));
        assert_eq!(f2, f1.pow(e));
    }

    #[test]
    fn pow_neg() {
        let f1 = FieldElement::new(BigInt::from(3), BigInt::from(13));
        let e = -9;
        let f2 = FieldElement::new(BigInt::from(1), BigInt::from(13));
        assert_eq!(f2, f1.pow(e));
    }

    #[test]
    fn add() {
        let f1 = FieldElement::new(BigInt::from(9), BigInt::from(13));
        let f2 = FieldElement::new(BigInt::from(11), BigInt::from(13));
        let f3 = FieldElement::new(BigInt::from(7), BigInt::from(13));
        assert_eq!(f1 + f2, f3);
    }

    #[test]
    fn sub() {
        let f1 = FieldElement::new(BigInt::from(3), BigInt::from(13));
        let f2 = FieldElement::new(BigInt::from(4), BigInt::from(13));
        let f3 = FieldElement::new(BigInt::from(1), BigInt::from(13));
        assert_eq!(f2 - f1, f3);
    }

    #[test]
    fn sub_overflow() {
        let f1 = FieldElement::new(BigInt::from(3), BigInt::from(13));
        let f2 = FieldElement::new(BigInt::from(4), BigInt::from(13));
        let f3 = FieldElement::new(BigInt::from(12), BigInt::from(13));
        assert_eq!(f1 - f2, f3);
    }

    #[test]
    fn mul() {
        let f1 = FieldElement::new(BigInt::from(3), BigInt::from(13));
        let f2 = FieldElement::new(BigInt::from(4), BigInt::from(13));
        let f3 = FieldElement::new(BigInt::from(12), BigInt::from(13));
        assert_eq!(f1 * f2, f3);
    }

    #[test]
    fn div() {
        let f1 = FieldElement::new(BigInt::from(2), BigInt::from(13));
        let f2 = FieldElement::new(BigInt::from(4), BigInt::from(13));
        let f3 = FieldElement::new(BigInt::from(7), BigInt::from(13));
        assert_eq!(f1 / f2, f3);
    }
}
