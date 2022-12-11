use crate::Solution;
use ndarray::{s, Array2, Axis};
use std::collections::HashMap;

fn parse(val: &str) -> Array2<bool> {
    let data: Vec<_> = val
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect();

    let size = (data.len() as f64).sqrt() as usize;

    Array2::from_shape_vec((size, size), data).unwrap()
}

fn expand(
    pattern: &Array2<bool>,
    replacements: &HashMap<Array2<bool>, Array2<bool>>,
) -> Array2<bool> {
    let oldsize = pattern.shape()[0] as isize;
    let oldstep = if oldsize % 2 == 0 { 2 } else { 3 };
    let newsize = oldsize * (oldstep + 1) / oldstep;
    let newstep = oldstep + 1;

    let mut result = Array2::default((newsize as usize, newsize as usize));

    for (i1, i2) in (0..oldsize)
        .step_by(oldstep as usize)
        .zip((0..newsize).step_by(newstep as usize))
    {
        for (j1, j2) in (0..oldsize)
            .step_by(oldstep as usize)
            .zip((0..newsize).step_by(newstep as usize))
        {
            let source = pattern.slice(s![i1..(i1 + oldstep), j1..(j1 + oldstep)]);

            if let Some(target) = replacements.get(&source.to_owned()) {
                result
                    .slice_mut(s![i2..(i2 + newstep), j2..(j2 + newstep)])
                    .assign(target);
            }
        }
    }

    result
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();
    let mut replacements = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let parts: Vec<&str> = line.split(" => ").collect();
        let mut source = parse(parts[0]);
        let target = parse(parts[1]);

        for _ in 0..4 {
            replacements.entry(source.clone()).or_insert(target.clone());
            source = source.reversed_axes();
            replacements.entry(source.clone()).or_insert(target.clone());
            source.invert_axis(Axis(0));
        }
    }

    let mut pattern = parse(".#./..#/###");

    for _i in 0..5 {
        pattern = expand(&pattern, &replacements);
    }

    println!(
        "[Part 1] Pixels on after 5 iterations: {}",
        pattern.iter().filter(|&&cell| cell).count()
    );

    for _i in 5..18 {
        pattern = expand(&pattern, &replacements);
    }

    println!(
        "[Part 2] Pixels on after 18 iterations: {}",
        pattern.iter().filter(|&&cell| cell).count()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
