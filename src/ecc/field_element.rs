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
        let mut current = FieldElement::new(One::one(), self.prime.clone());
        let mut coef = self.clone();
        while e > Zero::zero() {
            if &e % BigInt::from(2u8) == One::one() {
                current = current.clone() * coef.clone();
            }
            coef = coef.clone() * coef.clone();
            e /= 2;
        }
        return current;
    }
}

// impl ops::Add for FieldElement {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         if self.prime != other.prime {
//             panic!("can't add")
//         };

//         let num = (self.num + other.num) % &self.prime;
//         return Self::new(num, self.prime);
//     }
// }

impl_ops::impl_op_ex!(+ |a: &FieldElement, b: &FieldElement| -> FieldElement {
    if &a.prime != &b.prime {
        panic!("can't add")
    };

    let num = (&a.num + &b.num) % &a.prime;
    return FieldElement{num, prime: a.prime.clone()};
});

impl ops::Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't sub")
        };

        let prime = &self.prime;
        let num = (self.num - other.num + prime) % prime;
        return Self::new(num, self.prime);
    }
}

impl ops::Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't mul")
        };

        let num = (self.num * other.num) % &self.prime;
        return Self::new(num, self.prime);
    }
}

impl ops::Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't div");
        };
        if other.num == Zero::zero() {
            panic!("can't div by zero");
        }

        let prime = &self.prime;
        // prime must be more than 2 because this means primary number
        let other_inverse = other.pow((prime - BigInt::from(2)).try_into().unwrap());
        let num = (self.num * other_inverse.num) % prime;
        return Self::new(num, self.prime);
    }
}

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
