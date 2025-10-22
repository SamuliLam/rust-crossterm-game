use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2d {
            x,
            y,
        }
    }

}

impl Point2d<f64> {
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round()
        }
    }

    pub fn to_u16(&self) -> Point2d<u16> {
        Point2d {
            x: self.x as u16,
            y: self.y as u16,
        }
    }

    pub fn normalize(&self) -> Point2d<f64>
    {
        let (x, y) = (self.x, self.y);
        let magnitude = (x * x + y * y).sqrt();
        Point2d {
            x: x / magnitude,
            y: y / magnitude,
        }
    }
}

impl<T: std::fmt::Display> Display for Point2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Mul<f64> for Point2d<f64> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Point2d::new(x, y)
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point2d<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Add for Point2d<f64> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2d<f64> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
