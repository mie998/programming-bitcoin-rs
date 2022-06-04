use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FieldElement {
    num: usize,
    prime: usize,
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> Self {
        if num > prime {
            panic!("num should not exceed prime")
        }
        if prime < 2 {
            panic!("prime should be more than 2 or equal to 2")
        }

        Self { num, prime }
    }

    pub fn pow(self, exponent: u32) -> Self {
        let e = exponent % (self.prime as u32 - 1);
        return Self::new(self.num.pow(e) % self.prime, self.prime);
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let num = (self.num + other.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let num = (self.num - other.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let num = (self.num * other.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let other_inverse = other.pow((self.prime - 2).try_into().unwrap());
        let num = (self.num * other_inverse.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn num_exceeds_prime() {
        FieldElement::new(8, 5);
    }

    #[test]
    fn add() {
        let f1 = FieldElement::new(3, 13);
        let f2 = FieldElement::new(4, 13);
        let f3 = FieldElement::new(7, 13);
        assert_eq!(f1 + f2, f3);
    }

    #[test]
    fn sub() {
        let f1 = FieldElement::new(3, 13);
        let f2 = FieldElement::new(4, 13);
        let f3 = FieldElement::new(1, 13);
        assert_eq!(f2 - f1, f3);
    }

    #[test]
    fn mul() {
        let f1 = FieldElement::new(3, 13);
        let f2 = FieldElement::new(4, 13);
        let f3 = FieldElement::new(12, 13);
        assert_eq!(f1 * f2, f3);
    }

    #[test]
    fn div() {
        let f1 = FieldElement::new(2, 13);
        let f2 = FieldElement::new(4, 13);
        let f3 = FieldElement::new(7, 13);
        assert_eq!(f1 / f2, f3);
    }
}
