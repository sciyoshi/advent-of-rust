use regex::Regex;

pub fn ints(str: &str) -> Vec<i64> {
    let re = Regex::new(r"-?\d+").unwrap();

    re.find_iter(str)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}
