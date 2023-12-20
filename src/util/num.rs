pub fn lcm<T: IntoIterator>(nums: T) -> usize
where
    T::Item: Into<usize>,
{
    nums.into_iter().fold(1, |acc, n| {
        let n: usize = n.into();
        acc * n / num::integer::gcd(acc, n)
    })
}
