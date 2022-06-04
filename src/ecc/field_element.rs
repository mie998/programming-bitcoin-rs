use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FieldElement {
    pub num: usize,
    pub prime: usize,
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> Self {
        if prime < 2 {
            panic!("prime should be more than 2 or equal to 2")
        }

        let n: usize = num.rem_euclid(prime);
        Self { num: n, prime }
    }

    pub fn pow(self, exponent: i32) -> Self {
        let mut e = exponent.rem_euclid((self.prime - 1) as i32) as u32;
        let mut ans = self.clone();
        while e > 0 {
            ans = ans * self;
            e -= 1;
        }
        return Self::new(ans.num, self.prime);
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't add")
        };

        let num = (self.num + other.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't sub")
        };

        if self.num < other.num {
            // !TODO: ここキャストしすぎで汚い
            let num = (self.num as isize - other.num as isize).rem_euclid(self.prime as isize);
            return Self::new(num as usize, self.prime);
        } else {
            let num = (self.num - other.num) % self.prime;
            return Self::new(num, self.prime);
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't mul")
        };

        let num = (self.num * other.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("can't div");
        };
        if other.num == 0 {
            panic!("can't div by zero");
        }

        let other_inverse = other.pow((self.prime - 2).try_into().unwrap());
        let num = (self.num * other_inverse.num) % self.prime;
        return Self::new(num, self.prime);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let f1 = FieldElement::new(9, 13);
        let f2 = FieldElement::new(11, 13);
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
    fn sub_overflow() {
        let f1 = FieldElement::new(3, 13);
        let f2 = FieldElement::new(4, 13);
        let f3 = FieldElement::new(12, 13);
        assert_eq!(f1 - f2, f3);
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
