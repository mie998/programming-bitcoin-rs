use super::field_element::FieldElement as FE;
use impl_ops::*;
use num_bigint::BigInt;
#[allow(unused_imports)]
use num_traits::{One, ToPrimitive, Zero};
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: Option<FE>,
    pub y: Option<FE>,
    pub a: FE,
    pub b: FE,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct N {
    pub value: BigInt,
}

impl Point {
    pub fn new(x: Option<FE>, y: Option<FE>, a: FE, b: FE) -> Point {
        match (&x, &y) {
            (None, _) | (_, None) => Point {
                x: None,
                y: None,
                a,
                b,
            },
            (Some(x), Some(y)) => {
                if y.clone().pow(BigInt::from(2u8))
                    != x.clone().pow(BigInt::from(3u8)) + &a * x + &b
                {
                    panic!("Points {:?} {:?} are not on the curve!", x, y);
                }
                Point {
                    x: Some(x.clone()),
                    y: Some(y.clone()),
                    a,
                    b,
                }
            }
        }
    }

    pub fn rmul(self, coefficient: BigInt) -> Self {
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        let mut result = Self::new(None, None, self.a, self.b);

        while coef > BigInt::zero() {
            if &coef % BigInt::from(2u8) == BigInt::one() {
                result = &result + &current;
            };
            current = &current + &current;
            coef /= BigInt::from(2u8);
        }

        return result;
    }
}

impl_ops::impl_op_ex!(+ |p1: &Point, p2: &Point| -> Point{
        // case: Points are n qot on the same curve
        if p1.a != p2.a || p1.b != p2.b {
            panic!("Points {:?} {:?} are not on the same curve!", p1, p2);
        };

        // case: Points are on the same curve and meet a special condition
        if p1 == p2 {
            let (p1_x, p1_y) = (p1.x.as_ref().unwrap(), p1.y.as_ref().unwrap());
            if p1_x.clone() == FE::new(BigInt::zero(), p1_x.prime.clone()) * p1_y {
                return Point::new(None, None, p1.a.clone(), p1.b.clone());
            } else {
                let p = &p1_x.prime;
                let s = (FE::new(BigInt::from(3u8), p.clone()) * p1_x.clone().pow(BigInt::from(2u8)) + &p1.a) / (FE::new(BigInt::from(2u8), p.clone()) * p1_y);
                let x = s.clone().pow(BigInt::from(2u8)) - FE::new(BigInt::from(2u8), p.clone()) * p1_x;
                let y = s * (p1_x - x.clone()) - p1_y;
                return Point::new(Some(x), Some(y), p1.a.clone(), p1.b.clone());
            }
        }

        // case: Points are on the same curve and on normal positions
        match (&p1.x, &p2.x) {
            (None, _) => p2.clone(),
            (_, None) => p1.clone(),
            (Some(p1_x), Some(p2_x)) => {
                if p1_x == p2_x {
                    Point::new(None, None, p1.a.clone(), p1.b.clone())
                } else {
                    let (p1_y, p2_y) = (p1.y.as_ref().unwrap(), p2.y.as_ref().unwrap());
                    let s = (p2_y - p1_y) / (p2_x - p1_x);
                    let x = s.clone().pow(BigInt::from(2u8)) - p1_x - p2_x;
                    let y = s * (p1_x - x.clone()) - p1_y;
                    Point::new(Some(x), Some(y), p1.a.clone(), p1.b.clone())
                }
            }
        }
});

impl N {
    pub fn new() -> Self {
        let bytes_n = b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
        let n = BigInt::parse_bytes(bytes_n, 16).unwrap();
        Self { value: n }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn prime() -> BigInt {
        BigInt::from(7u8)
    }
    fn zero() -> FE {
        FE::new(BigInt::zero(), prime())
    }
    fn one() -> FE {
        FE::new(BigInt::from(1u8), prime())
    }
    fn two() -> FE {
        FE::new(BigInt::from(2u8), prime())
    }
    fn five() -> FE {
        FE::new(BigInt::from(5u8), prime())
    }
    fn seven() -> FE {
        FE::new(BigInt::from(7u8), prime())
    }
    fn neg_one() -> FE {
        zero() - one()
    }

    #[test]
    fn rmul() {
        let pn = BigInt::from(223u16);
        let x = Some(FE::new(BigInt::from(47u8), pn.clone()));
        let y = Some(FE::new(BigInt::from(71u8), pn.clone()));
        let a = FE::new(BigInt::zero(), pn.clone());
        let b = FE::new(BigInt::from(7u8), pn.clone());
        let p = Point::new(x, y, a, b);
        let mut v: Vec<(usize, usize)> = vec![];
        for i in 1..6 {
            let result = p.clone().rmul(BigInt::from(i));
            v.push((
                result.x.unwrap().num.to_usize().unwrap(),
                result.y.unwrap().num.to_usize().unwrap(),
            ));
        }
        let answers = vec![(47, 71), (36, 111), (15, 137), (194, 51), (126, 96)];
        assert_eq!(v, answers);
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
        let _p3 = p1 + p2;
    }

    #[test]
    fn add_fst_is_infinite() {
        let p1 = Point::new(None, None, five(), seven());
        let p2 = Point::new(Some(neg_one()), Some(one()), five(), seven());
        let p3 = &p1 + &p2;
        assert_eq!(p3, p2)
    }

    #[test]
    fn add_snd_is_infinite() {
        let p1 = Point::new(Some(neg_one()), Some(one()), five(), seven());
        let p2 = Point::new(None, None, five(), seven());
        let p3 = &p1 + &p2;
        assert_eq!(p3, p1)
    }

    #[test]
    fn add_normal() {
        let p1 = Point::new(Some(neg_one()), Some(neg_one()), five(), seven());
        let p2 = Point::new(Some(two()), Some(five()), five(), seven());
        let p3 = &p1 + &p2;
        assert_eq!(
            p3,
            Point::new(
                Some(FE::new(BigInt::from(3u8), prime())),
                Some(zero() - FE::new(BigInt::from(7u8), prime())),
                p1.a,
                p2.b
            )
        )
    }

    #[test]
    fn add_same_x() {
        let p1 = Point::new(Some(neg_one()), Some(one()), five(), seven());
        let p2 = Point::new(Some(neg_one()), Some(neg_one()), five(), seven());
        let p3 = p1 + p2;
        assert_eq!(p3.x, None);
        assert_eq!(p3.y, None);
    }

    #[test]
    fn add_double_roots() {
        let p1 = Point::new(Some(neg_one()), Some(neg_one()), five(), seven());
        let p2 = Point::new(Some(neg_one()), Some(neg_one()), five(), seven());
        let p3 = &p1 + &p2;
        assert_eq!(
            p3,
            Point::new(
                Some(FE::new(BigInt::from(18u8), prime())),
                Some(FE::new(BigInt::from(77u8), prime())),
                p1.a,
                p2.b
            )
        )
    }

    #[test]
    fn add_triple_roots() {
        let p1 = Point::new(Some(zero()), Some(one()), neg_one(), one());
        let p2 = Point::new(Some(zero()), Some(one()), neg_one(), one());
        let p3 = p1 + p2;
        assert_eq!(p3, Point::new(None, None, neg_one(), one()));
    }
}
