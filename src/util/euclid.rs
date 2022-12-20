#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pt<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Pt<T, N> {
    pub fn zero() -> Self
    where
        T: num::Zero + Copy,
    {
        Pt([T::zero(); N])
    }

    pub fn _unit(dir: usize) -> Self
    where
        T: num::Zero + num::One + Copy,
    {
        let mut result = Pt::zero();
        result[dir] = T::one();
        result
    }

    pub fn _norm1(&self) -> T
    where
        T: num::Signed + Copy,
    {
        self.0
            .iter()
            .map(|&e| e.abs())
            .reduce(|acc, v| acc + v)
            .unwrap()
    }

    pub fn _norm2(&self) -> f64
    where
        T: num::Signed + num::ToPrimitive + Copy,
    {
        self.0
            .iter()
            .map(|&e| e * e)
            .reduce(|acc, v| acc + v)
            .unwrap()
            .to_f64()
            .unwrap()
            .sqrt()
    }

    pub fn _normi(&self) -> T
    where
        T: num::Signed + std::cmp::Ord + Copy,
    {
        self.0.iter().map(|&e| e.abs()).max().unwrap()
    }

    pub fn nb_ortho(&self) -> impl Iterator<Item = Self>
    where
        T: num::Zero + num::One + std::ops::Neg<Output = T> + Copy,
    {
        let pt = self.clone();
        (0..N).flat_map(move |e| {
            [T::one().neg(), T::one()].map(move |s| {
                let mut result = pt.clone();
                result.0[e] = result.0[e] + s;
                result
            })
        })
    }

    // pub fn nb_normi(&self) -> impl Iterator<Item = Self> + '_ {
    //     (0..N).map(|i| )
    // }
}

impl<T, const N: usize, Idx> std::ops::Index<Idx> for Pt<T, N>
where
    Idx: std::slice::SliceIndex<[T], Output = T>,
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize, Idx> std::ops::IndexMut<Idx> for Pt<T, N>
where
    Idx: std::slice::SliceIndex<[T], Output = T>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const N: usize> From<Vec<T>> for Pt<T, N>
where
    T: num::Signed + Ord + Copy + num::ToPrimitive + std::fmt::Debug,
{
    fn from(value: Vec<T>) -> Self {
        let mut result = Self::zero();
        for i in 0..N {
            result[i] = value[i];
        }
        result
    }
}

impl<T: num::Signed + Ord + Copy + num::ToPrimitive, const N: usize> std::ops::Add for Pt<T, N> {
    type Output = Pt<T, N>;

    fn add(mut self, other: Self) -> Self::Output {
        self.0
            .iter_mut()
            .zip(other.0)
            .for_each(|(s, o)| *s = *s + o);

        Pt(self.0)
    }
}

impl<T: num::Signed + Ord + Copy + std::ops::AddAssign + num::ToPrimitive, const N: usize>
    std::ops::AddAssign for Pt<T, N>
{
    fn add_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0).for_each(|(s, o)| *s = *s + o);
    }
}

// impl<T: Signed + Ord + Copy + ToPrimitive> Sub for Pt<T> {
//     type Output = Pt<T>;

//     fn sub(self, other: Self) -> Self {
//         Pt(self.0 - other.0, self.1 - other.1)
//     }
// }

// impl<T: Signed + Ord + Copy + ToPrimitive> Mul<T> for Pt<T> {
//     type Output = Pt<T>;

//     fn mul(self, other: T) -> Self {
//         Pt(self.0 * other, self.1 * other)
//     }
// }

// impl<T: Signed + Ord + Copy + ToPrimitive> Div<T> for Pt<T> {
//     type Output = Pt<T>;

//     fn div(self, other: T) -> Self {
//         Pt(self.0 / other, self.1 / other)
//     }
// }

// impl<T: Signed + Ord + Copy + ToPrimitive> Neg for Pt<T> {
//     type Output = Pt<T>;

//     fn neg(self) -> Self {
//         Pt(-self.0, -self.1)
//     }
// }

#[cfg(test)]
mod tests {
    use super::Pt;

    #[test]
    fn test_pt_add() {
        assert!(Pt([1, 2, 3]) + Pt([4, 5, 6]) == Pt([5, 7, 9]));
    }
}
