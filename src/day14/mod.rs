use std::io::{self, BufRead};
use std::collections::HashSet;
use bit_vec::BitVec;
use super::day10::knothash;
use byteorder::{ByteOrder, BigEndian};
use util::Pt;

fn bits128(val: u128) -> BitVec {
	let mut bytes = [0u8; 16];
	BigEndian::write_u128(&mut bytes, val);
	BitVec::from_bytes(&bytes)
}

fn dfs(pts: &mut HashSet<Pt>, pt: Pt) {
	pts.remove(&pt);
	for nb in pt.nb4() {
		if pts.contains(&nb) {
			dfs(pts, nb);
		}
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let value = stdin.lock().lines().next().unwrap().unwrap();

	let rows: Vec<_> = (0..128)
		.map(|i| knothash(format!("{}-{}", value, i).bytes()))
		.collect();

	let mut pts: HashSet<Pt> = HashSet::new();

	for (i, &row) in rows.iter().enumerate() {
		for (j, bit) in bits128(row).iter().enumerate() {
			if bit {
				pts.insert(Pt(i as isize, j as isize));
			}
		}
	}

	println!("[Part 1] Count is: {}", pts.len());

	let mut count = 0;
	while let Some(&pt) = pts.iter().next() {
		dfs(&mut pts, pt);
		count += 1;
	}

	println!("[Part 2] Regions: {}", count);
}
