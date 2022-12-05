use regex::Regex;
use std::str::FromStr;

pub fn extract_integers(s: &str) -> Vec<i32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(s)
        .map(|x| i32::from_str(&x.as_str()))
        .filter_map(Result::ok)
        .collect()
}

pub fn extract_floats(s: &str) -> Vec<f64> {
    let re = Regex::new(r"-?\d+(\.\d+)?").unwrap();
    re.find_iter(s)
        .map(|x| f64::from_str(&x.as_str()))
        .filter_map(Result::ok)
        .collect()
}
