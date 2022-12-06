use num::Num;
use regex::Regex;
use std::str::FromStr;

pub fn extract_integers<T: Num>(s: &str) -> Vec<T> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(s)
        .map(|x| T::from_str_radix(&x.as_str(), 10))
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
