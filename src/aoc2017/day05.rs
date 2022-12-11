use crate::Solution;

fn run(mut data: Vec<isize>, updater: fn(isize) -> isize) -> usize {
    // Store a program counter (isize to allow negative)
    let mut pc = 0isize;
    let mut count = 0;

    while pc >= 0 && pc < data.len() as isize {
        let ins = data[pc as usize];
        data[pc as usize] += updater(ins);
        pc += ins;
        count += 1;
    }

    count
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    // Read into an array of instructions
    let data: Vec<_> = input
        .lines()
        .filter_map(|el| el.parse::<isize>().ok())
        .collect();

    let count1 = run(data.clone(), |_ins| 1);
    let count2 = run(data.clone(), |ins| if ins >= 3 { -1 } else { 1 });

    Solution(count1, count2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("0\n3\n0\n1\n-3") == crate::Solution(5, 10));
    }
}
