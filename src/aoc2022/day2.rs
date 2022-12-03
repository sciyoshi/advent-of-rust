use std::io::stdin;

pub fn solve() -> (u32, u32) {
    let mut result = (0, 0);

    for line in stdin().lines() {
        let (score1, score2) = match line.unwrap().as_ref() {
            "A X" => (1 + 3, 3 + 0),
            "A Y" => (2 + 6, 1 + 3),
            "A Z" => (3 + 0, 2 + 6),
            "B X" => (1 + 0, 1 + 0),
            "B Y" => (2 + 3, 2 + 3),
            "B Z" => (3 + 6, 3 + 6),
            "C X" => (1 + 6, 2 + 0),
            "C Y" => (2 + 0, 3 + 3),
            "C Z" => (3 + 3, 1 + 6),
            _ => panic!("invalid line")
        };

        result.0 += score1;
        result.1 += score2;
    }

    result
}