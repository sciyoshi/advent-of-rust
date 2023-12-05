// Generated with ChatGPT 4.

use crate::Solution;

fn parse_file(input: &str) -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    let mut current_map = Vec::new();

    for line in lines {
        if line.starts_with("seeds:") {
            seeds = line[7..]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
        } else if line.ends_with("map:") {
            if !current_map.is_empty() {
                maps.push(current_map.clone());
                current_map.clear();
            }
        } else if !line.is_empty() {
            let parts: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            if parts.len() == 3 {
                current_map.push((parts[0], parts[1], parts[2]));
            }
        }
    }
    if !current_map.is_empty() {
        maps.push(current_map);
    }

    (seeds, maps)
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    // Refactored to use parse_file
    let (seeds, maps) = parse_file(input);

    // Interpret seeds as individual ranges with length 1
    let seed_ranges_single: Vec<(usize, usize)> = seeds.iter().map(|&seed| (seed, 1)).collect();

    // Map all individual seeds through the mappings
    let mapped_ranges_single = map_all(seed_ranges_single, maps.clone());

    // Find the lowest element in the resulting ranges for individual seeds
    let lowest_element_single = mapped_ranges_single
        .iter()
        .map(|&(start, _)| start)
        .min()
        .unwrap_or(0);

    // Interpret seeds as a flattened list of (start, length) pairs
    let mut seed_ranges_pairs = Vec::new();
    for seed_pair in seeds.chunks_exact(2) {
        if let [start, length] = *seed_pair {
            seed_ranges_pairs.push((start, length));
        }
    }

    // Map all seed ranges through the mappings
    let mapped_ranges_pairs = map_all(seed_ranges_pairs, maps);

    // Find the lowest element in the resulting ranges for seed ranges
    let lowest_element_pairs = mapped_ranges_pairs
        .iter()
        .map(|&(start, _)| start)
        .min()
        .unwrap_or(0);

    Solution(lowest_element_single, lowest_element_pairs)
}

fn map_ranges(
    mut ranges: Vec<(usize, usize)>,
    mappings: &[(usize, usize, usize)],
) -> Vec<(usize, usize)> {
    let mut new_ranges = Vec::new();

    while !ranges.is_empty() {
        let (start, length) = ranges.remove(0); // Take the first range from the list
        let end = start + length;
        let current_start = start;

        let mut found_mapping = false;

        for &(dest_start, source_start, map_length) in mappings {
            let map_end = source_start + map_length;

            // Check for overlap with the current range
            if current_start < map_end && end > source_start {
                found_mapping = true;

                // Calculate the overlapping part
                let overlap_start = std::cmp::max(current_start, source_start);
                let overlap_end = std::cmp::min(end, map_end);
                let overlap_length = overlap_end - overlap_start;

                // Map the overlapping part
                let mapped_start = dest_start + (overlap_start - source_start);
                new_ranges.push((mapped_start, overlap_length));

                // Add the non-overlapping parts back to the ranges for further processing
                if overlap_start > current_start {
                    ranges.push((current_start, overlap_start - current_start));
                }
                if overlap_end < end {
                    ranges.push((overlap_end, end - overlap_end));
                }

                break; // Move to the next range
            }
        }

        // If no mapping was found for this range, add it as is to the new ranges
        if !found_mapping {
            new_ranges.push((current_start, length));
        }
    }

    new_ranges
}

fn map_all(
    initial_ranges: Vec<(usize, usize)>,
    all_mappings: Vec<Vec<(usize, usize, usize)>>,
) -> Vec<(usize, usize)> {
    let mut current_ranges = initial_ranges;

    for mappings in all_mappings {
        current_ranges = map_ranges(current_ranges, &mappings);
    }

    current_ranges
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day05.txt")) == crate::Solution(35, 46));
    }
}
