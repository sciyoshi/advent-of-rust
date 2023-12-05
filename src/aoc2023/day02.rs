// Generated with ChatGPT 4.

use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut sum_of_possible_game_ids = 0;
    let mut total_power_of_minimum_sets = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: usize = parts[0].replace("Game ", "").parse().unwrap();
        let sets: Vec<&str> = parts[1].split("; ").collect();

        let mut max_red_for_game = 0;
        let mut max_green_for_game = 0;
        let mut max_blue_for_game = 0;

        let mut possible_game = true;

        for set in &sets {
            let cubes: Vec<&str> = set.split(", ").collect();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for cube in cubes {
                let details: Vec<&str> = cube.split(' ').collect();
                let count: usize = details[0].parse().unwrap();
                match details[1] {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => {}
                }
            }

            if red > 12 || green > 13 || blue > 14 {
                possible_game = false;
            }

            max_red_for_game = max_red_for_game.max(red);
            max_green_for_game = max_green_for_game.max(green);
            max_blue_for_game = max_blue_for_game.max(blue);
        }

        if possible_game {
            sum_of_possible_game_ids += game_id;
        }

        total_power_of_minimum_sets += max_red_for_game * max_green_for_game * max_blue_for_game;
    }

    Solution(sum_of_possible_game_ids, total_power_of_minimum_sets)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day02.txt")) == crate::Solution(8, 2286));
    }
}
