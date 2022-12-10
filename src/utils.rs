use num::{Num, Signed, ToPrimitive};
use regex::Regex;
use std::cmp::{max, Ord};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use std::str::FromStr;

pub fn extract_integers<T: Num>(s: &str) -> Vec<T> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(s)
        .map(|x| T::from_str_radix(&x.as_str(), 10))
        .filter_map(Result::ok)
        .collect()
}

pub fn extract_floats(s: &str) -> Vec<f64> {
    let re = Regex::new(r"-?\d+(\.\d+)?").unwrap();
    re.find_iter(s)
        .map(|x| f64::from_str(&x.as_str()))
        .filter_map(Result::ok)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Pt<T: Signed + Ord + Copy + ToPrimitive>(pub T, pub T);

impl<T: Signed + Ord + Copy + ToPrimitive> Pt<T> {
    pub fn zero() -> Self {
        Pt(T::zero(), T::zero())
    }

    pub fn n() -> Self {
        Pt(T::zero(), T::one())
    }

    pub fn ne() -> Self {
        Pt(T::one(), T::one())
    }

    pub fn e() -> Self {
        Pt(T::one(), T::zero())
    }

    pub fn se() -> Self {
        Pt(T::one(), -T::one())
    }

    pub fn s() -> Self {
        Pt(T::zero(), -T::one())
    }

    pub fn sw() -> Self {
        Pt(-T::one(), -T::one())
    }

    pub fn w() -> Self {
        Pt(-T::one(), T::zero())
    }

    pub fn nw() -> Self {
        Pt(-T::one(), T::one())
    }

    pub fn within(self, x1: T, y1: T, x2: T, y2: T) -> bool {
        self.0 >= x1 && self.0 <= x2 && self.1 >= y1 && self.1 <= y2
    }

    pub fn nb8(self) -> Vec<Pt<T>> {
        vec![
            self + Pt::n(),
            self + Pt::ne(),
            self + Pt::e(),
            self + Pt::se(),
            self + Pt::s(),
            self + Pt::sw(),
            self + Pt::w(),
            self + Pt::nw(),
        ]
    }

    pub fn nb4(self) -> Vec<Pt<T>> {
        vec![
            self + Pt::n(),
            self + Pt::e(),
            self + Pt::s(),
            self + Pt::w(),
        ]
    }

    pub fn norm0(&self) -> T {
        self.0.abs() + self.1.abs()
    }

    pub fn norm2(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).to_f64().unwrap().sqrt()
    }

    pub fn normi(&self) -> T {
        max(self.0.abs(), self.1.abs())
    }
}

impl<T: Signed + Ord + Copy + ToPrimitive> Add for Pt<T> {
    type Output = Pt<T>;

    fn add(self, other: Self) -> Self::Output {
        Pt(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Signed + Ord + Copy + AddAssign + ToPrimitive> AddAssign for Pt<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Signed + Ord + Copy + ToPrimitive> Sub for Pt<T> {
    type Output = Pt<T>;

    fn sub(self, other: Self) -> Self {
        Pt(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Signed + Ord + Copy + ToPrimitive> Mul<T> for Pt<T> {
    type Output = Pt<T>;

    fn mul(self, other: T) -> Self {
        Pt(self.0 * other, self.1 * other)
    }
}

impl<T: Signed + Ord + Copy + ToPrimitive> Div<T> for Pt<T> {
    type Output = Pt<T>;

    fn div(self, other: T) -> Self {
        Pt(self.0 / other, self.1 / other)
    }
}

impl<T: Signed + Ord + Copy + ToPrimitive> Neg for Pt<T> {
    type Output = Pt<T>;

    fn neg(self) -> Self {
        Pt(-self.0, -self.1)
    }
}
