use super::day10::knothash;
use crate::Solution;
use crate::utils::Pt;
use bit_vec::BitVec;
use byteorder::{BigEndian, ByteOrder};
use std::collections::HashSet;

fn bits128(val: u128) -> BitVec {
    let mut bytes = [0u8; 16];
    BigEndian::write_u128(&mut bytes, val);
    BitVec::from_bytes(&bytes)
}

fn dfs(pts: &mut HashSet<Pt<i32>>, pt: Pt<i32>) {
    pts.remove(&pt);
    for nb in pt.nb4() {
        if pts.contains(&nb) {
            dfs(pts, nb);
        }
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let rows: Vec<_> = (0..128)
        .map(|i| knothash(format!("{}-{}", input, i).bytes()))
        .collect();

    let mut pts: HashSet<Pt<i32>> = HashSet::new();

    for (i, &row) in rows.iter().enumerate() {
        for (j, bit) in bits128(row).iter().enumerate() {
            if bit {
                pts.insert(Pt(i as i32, j as i32));
            }
        }
    }

    let part1 = pts.len();

    let mut count = 0;
    while let Some(&pt) = pts.iter().next() {
        dfs(&mut pts, pt);
        count += 1;
    }

    Solution(part1, count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("flqrgnkx") == crate::Solution(8108, 1242));
    }
}
