use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    Left(i32),
    Up(i32),
}

impl Fold {
    fn transform(&self, pt: (i32, i32)) -> (i32, i32) {
        match self {
            Fold::Left(num) => {
                if pt.0 > *num {
                    (2 * num - pt.0, pt.1)
                } else {
                    (pt.0, pt.1)
                }
            }
            Fold::Up(num) => {
                if pt.1 > *num {
                    (pt.0, 2 * num - pt.1)
                } else {
                    (pt.0, pt.1)
                }
            }
        }
    }
}

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let folds = data
        .iter()
        .filter(|line| line.starts_with("fold"))
        .map(|line| line.split_whitespace().nth(2).unwrap())
        .map(|line| {
            let mut split = line.split("=");
            let direction = split.next().unwrap();
            let num = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "y" => Fold::Up(num),
                "x" => Fold::Left(num),
                _ => panic!("Unknown fold direction"),
            }
        })
        .collect::<Vec<_>>();

    let points = data
        .iter()
        .filter(|line| line.contains(","))
        .map(|line| {
            let mut split = line.split(",");
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let first = points
        .iter()
        .map(|point| folds[0].transform(*point))
        .collect::<HashSet<_>>();

    let code = points
        .iter()
        .map(|point| folds.iter().fold(*point, |pt, fold| fold.transform(pt)))
        .collect::<HashSet<_>>();

    let width = code.iter().map(|pt| pt.0).max().unwrap();
    let height = code.iter().map(|pt| pt.1).max().unwrap();

    println!("[Part 1] {:?}", first.len());
    println!("[Part 2]");

    for j in 0..=height {
        for i in 0..=width {
            if code.contains(&(i, j)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}
