// Generated with ChatGPT
// https://chat.openai.com/share/56ff2a91-2b6a-42e6-b400-34f45ec5fbc1

use crate::Solution;
use std::collections::HashMap;

struct Lens {
    label: String,
    focal_length: usize,
}

fn hash(input: &str) -> usize {
    let mut current_value: usize = 0;

    for c in input.chars() {
        let ascii = c as usize;
        current_value = (current_value + ascii) * 17 % 256;
    }

    current_value
}
fn part2(input: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
    let steps = input.split(',');

    for step in steps {
        let (label, operation, value) = parse_step(step);
        let box_number = hash(&label);

        match operation {
            '-' => {
                if let Some(lenses) = boxes.get_mut(&box_number) {
                    lenses.retain(|lens| lens.label != label);
                }
            }
            '=' => {
                let focal_length = value.unwrap();
                let lens = Lens {
                    label: label.clone(),
                    focal_length,
                };

                let lenses = boxes.entry(box_number).or_insert_with(Vec::new);
                if let Some(index) = lenses.iter().position(|l| l.label == label) {
                    lenses[index] = lens; // Replace the lens at the same position
                } else {
                    lenses.push(lens); // Add new lens to the end if it doesn't exist
                }
            }
            _ => unreachable!(),
        }
    }

    calculate_focusing_power(&boxes)
}
fn parse_step(step: &str) -> (String, char, Option<usize>) {
    let (label, rest) = step.split_at(step.find(|c: char| c == '-' || c == '=').unwrap());
    let operation = rest.chars().next().unwrap();
    let value = if operation == '=' {
        Some(rest[1..].parse::<usize>().unwrap())
    } else {
        None
    };

    (label.to_string(), operation, value)
}

fn calculate_focusing_power(boxes: &HashMap<usize, Vec<Lens>>) -> usize {
    let mut total_power = 0;

    for (box_number, lenses) in boxes {
        for (slot, lens) in lenses.iter().enumerate() {
            total_power += (1 + box_number) * (slot + 1) * lens.focal_length;
        }
    }

    total_power
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let steps = input.split(',');
    let mut sum = 0;

    for step in steps {
        sum += hash(step);
    }

    Solution(sum, part2(input))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day15.txt")) == crate::Solution(1320, 145));
    }
}
