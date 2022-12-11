use crate::Solution;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Cmd {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn permute(mut vals: Vec<char>, cmds: &[Cmd]) -> Vec<char> {
    let len = vals.len();

    for ref cmd in cmds {
        match *cmd {
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

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let cmds: Vec<Cmd> = line
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

    println!(
        "[Part 1] Order is: {}",
        permute(vals.clone(), &cmds).iter().collect::<String>()
    );

    let mut cycle = vec![vals.clone()];

    loop {
        vals = permute(vals, &cmds);

        if vals == cycle[0] {
            break;
        }

        cycle.push(vals.clone());
    }

    println!(
        "[Part 2] Order is: {}",
        cycle[1_000_000_000 % cycle.len()]
            .iter()
            .collect::<String>()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
