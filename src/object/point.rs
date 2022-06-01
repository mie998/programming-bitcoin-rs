use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point<T> {
    a: T,
    b: T,
    x: T,
    y: T,
}
impl<T> Point<T> {
    pub fn new(a: T, b: T, x: T, y: T) -> Option<Self<T>> {
        // if (a) {}
        if y.pow(2) != x.pow(3) + a * x + b {
            panic!("this is not a curve pa");
        }
        return Some(Point { a, b, x, y });
    }
}
