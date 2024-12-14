use crate::{Solution, utils::extract_integers};

#[derive(Debug, Copy, Clone)]
struct Machine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    px: f64,
    py: f64,
}

impl Machine {
    pub fn tokens(&self, part2: bool) -> Option<usize> {
        let px = self.px + if part2 { 10000000000000f64 } else { 0f64 };
        let py = self.py + if part2 { 10000000000000f64 } else { 0f64 };
        let a = (self.by * px - self.bx * py) / (self.ax * self.by - self.ay * self.bx);
        let b = (self.ay * px - self.ax * py) / (self.ay * self.bx - self.ax * self.by);
        if a.fract() == 0f64 && b.fract() == 0f64 {
            Some(3 * (a as usize) + (b as usize))
        } else {
            None
        }
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(extract_integers::<u32>)
        .map(|els| Machine {
            ax: els[0] as f64,
            ay: els[1] as f64,
            bx: els[2] as f64,
            by: els[3] as f64,
            px: els[4] as f64,
            py: els[5] as f64,
        })
        .collect();

    let tokens: usize = machines.iter().flat_map(|m| m.tokens(false)).sum();
    let tokens2: usize = machines.iter().flat_map(|m| m.tokens(true)).sum();

    Solution(tokens as usize, tokens2 as usize)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day13.txt")) == crate::Solution(480, 875318608908)
        );
    }
}
