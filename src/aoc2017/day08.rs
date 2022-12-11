use crate::Solution;
use std::cmp;
use std::collections::HashMap;

fn parse_inst(inst: &str) -> (&str, &str, i64, &str, &str, i64) {
    let mut split = inst.split_ascii_whitespace();
    let reg1 = split.next().unwrap();
    let op1 = split.next().unwrap();
    let val = split.next().unwrap().parse().unwrap();
    split.next().unwrap();
    let reg2 = split.next().unwrap();
    let op2 = split.next().unwrap();
    let val2 = split.next().unwrap().parse().unwrap();

    (reg1, op1, val, reg2, op2, val2)
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let mut regs = HashMap::<String, i64>::new();
    let mut maxseen = 0;

    for line in input.lines() {
        let (reg1, op1, val1, reg2, op2, val2) = parse_inst(line);
        let reg2 = *regs.get(reg2).unwrap_or(&0);

        let cond = match op2 {
            "<=" => reg2 <= val2,
            ">=" => reg2 >= val2,
            "<" => reg2 < val2,
            ">" => reg2 > val2,
            "==" => reg2 == val2,
            "!=" => reg2 != val2,
            _ => panic!("unknown operator"),
        };

        if cond {
            let reg1 = regs.entry(reg1.to_string()).or_insert(0);

            match op1 {
                "inc" => *reg1 += val1,
                "dec" => *reg1 -= val1,
                _ => panic!("unknown operator"),
            }

            maxseen = cmp::max(maxseen, *reg1);
        }
    }

    Solution(regs.values().cloned().max().unwrap(), maxseen)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day08.txt")) == crate::Solution(1, 10));
    }
}
