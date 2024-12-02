use crate::Solution;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cmd {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn permute(mut vals: Vec<char>, cmds: &[Cmd]) -> Vec<char> {
    let len = vals.len();

    for cmd in cmds {
        match cmd {
            Cmd::Spin(val) => vals.rotate_left(len - val),
            Cmd::Exchange(i, j) => vals.swap(*i, *j),
            Cmd::Partner(i, j) => {
                let pos1 = vals.iter().position(|c| c == i).unwrap();
                let pos2 = vals.iter().position(|c| c == j).unwrap();
                vals.swap(pos1, pos2);
            }
        }
    }

    vals
}

pub fn solve(input: &str) -> Solution<String, String> {
    let cmds: Vec<Cmd> = input
        .trim()
        .split(",")
        .map(|cmd| {
            let (op, arg) = cmd.split_at(1);
            let args: Vec<&str> = arg.split("/").collect();
            match op {
                "s" => Cmd::Spin(args[0].parse().unwrap()),
                "x" => Cmd::Exchange(args[0].parse().unwrap(), args[1].parse().unwrap()),
                "p" => Cmd::Partner(args[0].parse().unwrap(), args[1].parse().unwrap()),
                _ => panic!("unknown command"),
            }
        })
        .collect();

    let mut vals: Vec<char> = "abcdefghijklmnop".chars().collect();

    let part1 = permute(vals.clone(), &cmds).iter().collect::<String>();

    let mut cycle = vec![vals.clone()];

    loop {
        vals = permute(vals, &cmds);

        if vals == cycle[0] {
            break;
        }

        cycle.push(vals.clone());
    }

    let part2 = cycle[1_000_000_000 % cycle.len()]
        .iter()
        .collect::<String>();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("s1,x3/4,pe/b").0 == *"paedcbfghijklmno");
    }
}
