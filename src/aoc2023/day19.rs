// Partially generated with ChatGPT
// https://chat.openai.com/share/bec23472-9cb2-4fb6-86f3-dccfab2e494e

use crate::Solution;

use std::collections::HashMap;

#[derive(Debug)]
enum Cmp {
    Gt, // Greater than
    Lt, // Less than
}

#[derive(Debug)]
struct Condition {
    attribute: char, // 'x', 'm', 'a', or 's'
    cmp: Cmp,
    value: usize,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    action: String,
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse_condition(cond_str: &str) -> Condition {
    let attribute = cond_str.chars().next().unwrap();
    let cmp = if cond_str.contains('>') {
        Cmp::Gt
    } else {
        Cmp::Lt
    };
    let value = cond_str
        .split(['>', '<'])
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();

    Condition {
        attribute,
        cmp,
        value,
    }
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let sections: Vec<&str> = input.split("\n\n").collect();
    let workflow_section = sections[0];
    let part_section = sections[1];

    // Parse workflows
    for line in workflow_section.lines() {
        let parts: Vec<&str> = line.split('{').collect();
        let workflow_name = parts[0].to_string();
        let rules_str = parts[1].trim_end_matches('}');
        let mut rules = Vec::new();

        for rule_str in rules_str.split(',') {
            let parts: Vec<&str> = rule_str.split(':').collect();
            let condition = if parts.len() > 1 {
                Some(parse_condition(parts[0]))
            } else {
                None
            };
            let action = parts[parts.len() - 1].to_string();
            rules.push(Rule { condition, action });
        }

        workflows.insert(workflow_name, rules);
    }

    // Parse parts
    for line in part_section.lines() {
        let ratings: Vec<&str> = line
            .trim_matches(|p| p == '{' || p == '}')
            .split(',')
            .collect();
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for rating in ratings {
            let parts: Vec<&str> = rating.split('=').collect();
            let value = parts[1].parse::<usize>().unwrap();

            match parts[0] {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => {}
            }
        }

        parts.push(part);
    }

    (workflows, parts)
}

fn apply(workflows: &HashMap<String, Vec<Rule>>, part: Part) -> bool {
    let mut current_workflow = "in"; // Assuming all parts start in the "in" workflow

    loop {
        if let Some(rules) = workflows.get(current_workflow) {
            for rule in rules {
                if let Some(condition) = &rule.condition {
                    let value = match condition.attribute {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => continue,
                    };

                    let matched = match condition.cmp {
                        Cmp::Gt => value > condition.value,
                        Cmp::Lt => value < condition.value,
                    };

                    if matched {
                        if rule.action == "A" {
                            return true;
                        } else if rule.action == "R" {
                            return false;
                        } else {
                            current_workflow = &rule.action;
                            break;
                        }
                    }
                } else {
                    // No condition means automatic match
                    if rule.action == "A" {
                        return true;
                    } else if rule.action == "R" {
                        return false;
                    } else {
                        current_workflow = &rule.action;
                        break;
                    }
                }
            }
        } else {
            // If the workflow doesn't exist, it's an error state, treating as rejection
            return false;
        }
    }
}

const N: usize = 4; // Example dimension, can be changed

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cuboid {
    min: [usize; N],
    max: [usize; N],
}

fn split_range(
    cuboid: Cuboid,
    dimension: usize,
    condition: &Condition,
) -> (Option<Cuboid>, Option<Cuboid>) {
    let (min, max) = (cuboid.min[dimension], cuboid.max[dimension]);

    match condition.cmp {
        Cmp::Lt => {
            if max < condition.value {
                (Some(cuboid), None)
            } else if min >= condition.value {
                (None, Some(cuboid))
            } else {
                let mut cuboid1 = cuboid;
                let mut cuboid2 = cuboid;
                cuboid1.max[dimension] = condition.value - 1;
                cuboid2.min[dimension] = condition.value;
                (Some(cuboid1), Some(cuboid2))
            }
        }
        Cmp::Gt => {
            if min > condition.value {
                (Some(cuboid), None)
            } else if max <= condition.value {
                (None, Some(cuboid))
            } else {
                let mut cuboid1 = cuboid;
                let mut cuboid2 = cuboid;
                cuboid1.min[dimension] = condition.value + 1;
                cuboid2.max[dimension] = condition.value;
                (Some(cuboid1), Some(cuboid2))
            }
        }
    }
}

fn explore(
    workflow_name: &str,
    mut range: Cuboid,
    workflows: &HashMap<String, Vec<Rule>>,
) -> usize {
    let mut total = 0;
    if let Some(rules) = workflows.get(workflow_name) {
        for rule in rules {
            let mut new_range = range;

            // Update new_range based on the rule's condition
            if let Some(condition) = &rule.condition {
                let (range1, range2) = match condition.attribute {
                    'x' => split_range(new_range, 0, condition),
                    'm' => split_range(new_range, 1, condition),
                    'a' => split_range(new_range, 2, condition),
                    's' => split_range(new_range, 3, condition),
                    _ => continue,
                };

                if range1.is_some() {
                    new_range = range1.unwrap();
                }

                if range2.is_some() {
                    range = range2.unwrap()
                }
            }

            if rule.action == "A" {
                total += (new_range.max[0] - new_range.min[0] + 1)
                    * (new_range.max[1] - new_range.min[1] + 1)
                    * (new_range.max[2] - new_range.min[2] + 1)
                    * (new_range.max[3] - new_range.min[3] + 1);
            } else if rule.action != "R" {
                total += explore(&rule.action, new_range, workflows);
            }
        }
    }
    total
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let (workflows, parts) = parse(input);

    let mut total = 0;
    for part in parts {
        if apply(&workflows, part) {
            total += part.x + part.m + part.a + part.s;
        }
    }

    let part2 = explore(
        "in",
        Cuboid {
            min: [1, 1, 1, 1],
            max: [4000, 4000, 4000, 4000],
        },
        &workflows,
    );

    Solution(total, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day19.txt"))
                == crate::Solution(19114, 167409079868000)
        );
    }
}
