use crate::Solution;

#[derive(Debug, Clone, Copy)]
enum ParseResult {
    Complete,
    Incomplete(u64),
    Corrupted(char),
}

fn parse(i: &str) -> ParseResult {
    let mut stack = vec![];

    for c in i.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            ']' | ')' | '}' | '>' => {
                if let Some(x) = stack.pop() {
                    match (x, c) {
                        ('[', ']') | ('(', ')') | ('{', '}') | ('<', '>') => (),
                        _ => return ParseResult::Corrupted(c),
                    }
                }
            }
            _ => (),
        }
    }

    if stack.is_empty() {
        ParseResult::Complete
    } else {
        let mut score = 0;
        while let Some(x) = stack.pop() {
            score *= 5;
            score += match x {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
        }
        ParseResult::Incomplete(score)
    }
}

pub fn solve(input: &str) -> Solution<usize, u64> {
    let data: Vec<_> = input.lines().collect();

    let mut part1 = 0;
    let mut part2 = vec![];

    for l in data {
        let parsed = parse(l);

        part1 += match parsed {
            ParseResult::Corrupted(')') => 3,
            ParseResult::Corrupted(']') => 57,
            ParseResult::Corrupted('}') => 1197,
            ParseResult::Corrupted('>') => 25137,
            _ => 0,
        };

        if let ParseResult::Incomplete(n) = parsed {
            part2.push(n)
        }
    }

    part2.sort();

    Solution(part1, part2[part2.len() / 2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day10.txt")) == crate::Solution(26397, 288957));
    }
}
