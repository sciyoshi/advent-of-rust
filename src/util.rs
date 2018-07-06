use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::max;
use nom::*;
use num::traits::Signed;

named!(pub num(&str) -> i64, do_parse!(
	s: recognize!(
		pair!(
			opt!(tag_s!("-")),
			call!(digit)
		)
	) >> (s.parse().unwrap())
));


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pt<T: Signed + Copy + Ord=isize>(pub T, pub T);

impl<T: Signed + Copy + Ord> Pt<T> {
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

	pub fn rot90r(self) -> Self {
		Pt(self.1, -self.0)
	}

	pub fn rot90l(self) -> Self {
		Pt(-self.1, self.0)
	}

	pub fn norm1(self) -> T {
		self.0.abs() + self.1.abs()
	}

	pub fn normi(self) -> T {
		max(self.0.abs(), self.1.abs())
	}

	pub fn nb4(self) -> Vec<Self> {
		vec![
			self + Pt::n(),
			self + Pt::e(),
			self + Pt::s(),
			self + Pt::w()
		]
	}

	pub fn nb8(self) -> Vec<Self> {
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
}

impl<T: Signed + Copy + Ord> Add for Pt<T> {
	type Output = Pt<T>;

	fn add(self, other: Self) -> Self::Output {
		Pt(self.0 + other.0, self.1 + other.1)
	}
}

impl<'a, T: Signed + Copy + Ord> Add<Pt<T>> for &'a Pt<T> {
	type Output = Pt<T>;

	fn add(self, other: Pt<T>) -> Self::Output {
		Pt(self.0 + other.0, self.1 + other.1)
	}
}

impl<T: Signed + Copy + Ord> Sub for Pt<T> {
	type Output = Pt<T>;

	fn sub(self, other: Self) -> Self {
		Pt(self.0 - other.0, self.1 - other.1)
	}
}

impl<T: Signed + Copy + Ord> Mul<T> for Pt<T> {
	type Output = Pt<T>;

	fn mul(self, other: T) -> Self {
		Pt(self.0 * other, self.1 * other)
	}
}

impl<T: Signed + Copy + Ord> Div<T> for Pt<T> {
	type Output = Pt<T>;

	fn div(self, other: T) -> Self {
		Pt(self.0 / other, self.1 / other)
	}
}

impl<T: Signed + Copy + Ord> Neg for Pt<T> {
	type Output = Pt<T>;

	fn neg(self) -> Self {
		Pt(-self.0, -self.1)
	}
}
