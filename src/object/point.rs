use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: Option<usize>,
    y: Option<usize>,
    a: usize,
    b: usize,
}

impl Point {
    pub fn new(x: Option<usize>, y: Option<usize>, a: usize, b: usize) -> Point {
        match (x, y) {
            (None, _) | (_, None) => Point {
                x: None,
                y: None,
                a,
                b,
            },
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3) + a * x + b {
                    panic!("Points {} {} are not on the curve!", x, y);
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
            if self_x == 0 * self_y {
                return Self::new(None, None, self.a, self.b);
            } else {
                // TODO: usize のままではいい感じに計算できていない可能性がある。
                let s = (3 * self_x.pow(2) + self.a) / (2 * self_y);
                let x = s.pow(2) - 2 * self_x;
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
                    // TODO: usize のままではいい感じに計算できていない可能性がある。
                    let s = (other_y - self_y) / (other_x - self_x);
                    let x = s.pow(2) - self_x - other_x;
                    let y = s * (self_x - x) - self_y;
                    Self::new(Some(x), Some(y), self.a, self.b)
                }
            }
        }
    }
}
