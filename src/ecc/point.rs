use super::field_element::FieldElement as FE;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: Option<FE>,
    y: Option<FE>,
    a: FE,
    b: FE,
}

impl Point {
    pub fn new(x: Option<FE>, y: Option<FE>, a: FE, b: FE) -> Point {
        match (x, y) {
            (None, _) | (_, None) => Point {
                x: None,
                y: None,
                a,
                b,
            },
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3) + a * x + b {
                    panic!("Points {:?} {:?} are not on the curve!", x, y);
                };
                Point {
                    x: Some(x),
                    y: Some(y),
                    a,
                    b,
                }
            }
        }
    }

    pub fn rmul(self, coefficient: isize) -> Self {
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        let mut result = Self::new(None, None, self.a, self.b);
        while coef > 0 {
            if coef & 1 == 1 {
                result = result + current;
            };
            current = current + current;
            coef >>= 1
        }
        return result;
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        // case: Points are not on the same curve
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?} {:?} are not on the same curve!", self, other);
        };

        // case: Points are on the same curve and meet a special condition
        if self == other {
            let (self_x, self_y) = (self.x.unwrap(), self.y.unwrap());
            if self_x == FE::new(0, self_x.prime) * self_y {
                return Self::new(None, None, self.a, self.b);
            } else {
                let p = self_x.prime;
                let s = (FE::new(3, p) * self_x.pow(2) + self.a) / (FE::new(2, p) * self_y);
                let x = s.pow(2) - FE::new(2, p) * self_x;
                let y = s * (self_x - x) - self_y;
                return Self::new(Some(x), Some(y), self.a, self.b);
            }
        }

        // case: Points are on the same curve and on normal positions
        match (self.x, other.x) {
            (None, _) => other,
            (_, None) => self,
            (Some(self_x), Some(other_x)) => {
                if self_x == other_x {
                    Self::new(None, None, self.a, self.b)
                } else {
                    let (self_y, other_y) = (self.y.unwrap(), other.y.unwrap());
                    let s = (other_y - self_y) / (other_x - self_x);
                    let x = s.pow(2) - self_x - other_x;
                    let y = s * (self_x - x) - self_y;
                    Self::new(Some(x), Some(y), self.a, self.b)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PRIME: usize = 7;
    fn zero() -> FE {
        FE::new(0, PRIME)
    }
    fn one() -> FE {
        FE::new(1, PRIME)
    }
    fn five() -> FE {
        FE::new(5, PRIME)
    }
    fn seven() -> FE {
        FE::new(7, PRIME)
    }
    fn neg_one() -> FE {
        zero() - one()
    }

    #[test]
    #[should_panic]
    fn not_on_curve() {
        Point::new(Some(one()), Some(one()), one(), one());
    }

    #[test]
    #[should_panic]
    fn add_not_on_same_curve() {
        let p1 = Point::new(Some(neg_one()), Some(one()), five(), seven());
        let p2 = Point::new(Some(one()), Some(one()), neg_one(), one());
        let p3 = p1 + p2;
    }

    // !TODO: test 続き
    // #[test]
    // fn add_normal_point_to_special() {
    //     let p1 = Point::new(Some(-1), Some(1), 5, 7);
    //     let p2 = Point::new(Some(-1), Some(1), 5, 7);
    //     let p2 = Point::new(Some(-1), Some(1), 5, 7);
    // }
}
