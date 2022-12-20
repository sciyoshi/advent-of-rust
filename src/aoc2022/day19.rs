use crate::{utils::extract_integers, Solution};

#[derive(Default, Clone, Debug)]
struct State {
    time_remaining: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl State {
    fn evolve(&mut self, time: usize) {
        self.time_remaining -= time;
        self.ore += time * self.ore_robots;
        self.clay += time * self.clay_robots;
        self.obsidian += time * self.obsidian_robots;
        self.geode += time * self.geode_robots;
    }
}

#[derive(Debug)]
pub struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian_ore: usize,
    obsidian_clay: usize,
    geode_ore: usize,
    geode_obsidian: usize,
}

impl Blueprint {
    fn best(&self, time: usize) -> usize {
        let mut stack: Vec<State> = vec![State {
            time_remaining: time,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }];
        let mut best: usize = 0;

        let max_ore = [self.ore, self.clay, self.obsidian_ore, self.geode_ore]
            .into_iter()
            .max()
            .unwrap();

        while let Some(state) = stack.pop() {
            let mut has_next = false;

            if state.time_remaining > 0 {
                if state.geode
                    + state.time_remaining * state.geode_robots
                    + (state.time_remaining * (state.time_remaining - 1)) / 2
                    < best
                {
                    continue;
                }

                if state.ore_robots < max_ore {
                    let step = if self.ore > state.ore {
                        (self.ore - state.ore + state.ore_robots - 1) / state.ore_robots + 1
                    } else {
                        1
                    };

                    if step <= state.time_remaining {
                        let mut state = state.clone();
                        state.evolve(step);
                        state.ore -= self.ore;
                        state.ore_robots += 1;
                        stack.push(state);
                        has_next = true;
                    }
                }

                if state.clay_robots < self.obsidian_clay {
                    let step = if self.clay > state.ore {
                        (self.clay - state.ore + state.ore_robots - 1) / state.ore_robots + 1
                    } else {
                        1
                    };

                    if step <= state.time_remaining {
                        let mut state = state.clone();
                        state.evolve(step);
                        state.ore -= self.clay;
                        state.clay_robots += 1;
                        stack.push(state);
                        has_next = true;
                    }
                }

                if state.clay_robots > 0 {
                    let step_ore = if self.obsidian_ore > state.ore {
                        (self.obsidian_ore - state.ore + state.ore_robots - 1) / state.ore_robots
                            + 1
                    } else {
                        1
                    };

                    let step_clay = if self.obsidian_clay > state.clay {
                        (self.obsidian_clay - state.clay + state.clay_robots - 1)
                            / state.clay_robots
                            + 1
                    } else {
                        1
                    };

                    let step = step_ore.max(step_clay);

                    if step <= state.time_remaining {
                        let mut state = state.clone();
                        state.evolve(step);
                        state.ore -= self.obsidian_ore;
                        state.clay -= self.obsidian_clay;
                        state.obsidian_robots += 1;
                        stack.push(state);
                        has_next = true;
                    }
                }

                if state.obsidian_robots > 0 {
                    let step_ore = if self.geode_ore > state.ore {
                        (self.geode_ore - state.ore + state.ore_robots - 1) / state.ore_robots + 1
                    } else {
                        1
                    };

                    let step_obsidian = if self.geode_obsidian > state.obsidian {
                        (self.geode_obsidian - state.obsidian + state.obsidian_robots - 1)
                            / state.obsidian_robots
                            + 1
                    } else {
                        1
                    };

                    let step = step_ore.max(step_obsidian);

                    if step <= state.time_remaining {
                        let mut state = state.clone();
                        state.evolve(step);
                        state.ore -= self.geode_ore;
                        state.obsidian -= self.geode_obsidian;
                        state.geode_robots += 1;
                        stack.push(state);
                        has_next = true;
                    }
                }
            }

            if !has_next {
                let geode = state.geode + state.geode_robots * state.time_remaining;
                if geode > best {
                    best = geode;
                }
            }
        }

        best
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let blueprints: Vec<_> = input
        .lines()
        .map(|line| {
            let parsed = extract_integers(line);

            Blueprint {
                id: parsed[0],
                ore: parsed[1],
                clay: parsed[2],
                obsidian_ore: parsed[3],
                obsidian_clay: parsed[4],
                geode_ore: parsed[5],
                geode_obsidian: parsed[6],
            }
        })
        .collect();

    let part1 = blueprints.iter().map(|b| b.id * b.best(24)).sum();
    let part2 = blueprints[0].best(32) * blueprints[1].best(32) * blueprints[2].best(32);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let bp1 = super::Blueprint {
            id: 1,
            ore: 4,
            clay: 2,
            obsidian_ore: 3,
            obsidian_clay: 14,
            geode_ore: 2,
            geode_obsidian: 7,
        };

        let bp2 = super::Blueprint {
            id: 2,
            ore: 2,
            clay: 3,
            obsidian_ore: 3,
            obsidian_clay: 8,
            geode_ore: 3,
            geode_obsidian: 12,
        };

        assert!(bp1.best(24) == 9);
        assert!(bp2.best(24) == 12);
        assert!(bp1.best(32) == 56);
        assert!(bp2.best(32) == 62);
    }
}
