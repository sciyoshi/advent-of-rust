use crate::Solution;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{is_not, tag, take},
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, pair},
};

#[derive(Clone)]
struct Stats {
    score: usize,
    garbage: usize,
}

fn garbage(input: &str) -> IResult<&str, usize> {
    delimited(
        tag("<"),
        fold_many0(
            alt((
                map(is_not("!>"), |v: &str| v.len()),
                value(0, pair(tag("!"), take(1u8))),
            )),
            || 0,
            |acc: usize, item: usize| acc + item,
        ),
        tag(">"),
    )(input)
}

fn group(input: &str, depth: usize) -> IResult<&str, Stats> {
    delimited(
        tag("{"),
        fold_many0(
            alt((
                |inp| group(inp, depth + 1),
                map(garbage, |len| Stats {
                    score: 0,
                    garbage: len,
                }),
                value(
                    Stats {
                        score: 0,
                        garbage: 0,
                    },
                    tag(","),
                ),
            )),
            || Stats {
                score: depth + 1,
                garbage: 0,
            },
            |acc: Stats, item: Stats| Stats {
                score: acc.score + item.score,
                garbage: acc.garbage + item.garbage,
            },
        ),
        tag("}"),
    )(input)
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let (_, stats) = group(input, 0).unwrap();

    Solution(stats.score, stats.garbage)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("{{{},{},{{}}}}").0 == 16);
        assert!(super::solve("{{<!!>},{<!!>},{<!!>},{<!!>}}").0 == 9);
        assert!(super::solve("{{<a!>},{<a!>},{<a!>},{<ab>}}").0 == 3);
        assert!(super::solve("{<random characters>}").1 == 17);
    }
}
