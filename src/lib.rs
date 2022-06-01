#[derive(Debug)]
pub struct FieldElement {
    num: usize,
    prime: usize,
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> Self {
        if num > prime {
            panic!("num should not exceed prime")
        }
        if prime <= 2 {
            panic!("prime should be more than 2 or equal to 2")
        }

        Self { num, prime }
    }

    pub fn eq(self, other: Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    pub fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let num = (self.num + other.num) % self.prime;
        return Self::new(num, self.prime);
    }

    pub fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let num = (self.num - other.num) % self.prime;
        return Self::new(num, self.prime);
    }

    pub fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let num = (self.num * other.num) % self.prime;
        return Self::new(num, self.prime);
    }

    pub fn dev(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cann't")
        };

        let other_inverse = other.pow((self.prime - 2).try_into().unwrap());
        let num = (self.num * other_inverse.num) % self.prime;
        return Self::new(num, self.prime);
    }

    pub fn pow(self, exponent: u32) -> Self {
        let e = exponent % (self.prime as u32 - 1);
        return Self::new(self.num.pow(e) % self.prime, self.prime);
    }
}
