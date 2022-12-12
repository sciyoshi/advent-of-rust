use itertools::Itertools;
use pathfinding::directed::astar::astar;
use std::{
    fmt::{self, Display, Formatter},
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Amphipod {
    fn hallway_pos(&self) -> usize {
        2 + 2 * *self as usize
    }

    fn cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Amphipod::A => "A",
                Amphipod::B => "B",
                Amphipod::C => "C",
                Amphipod::D => "D",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Burrow<const SIZE: usize = 2> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; SIZE]; 4],
}

const HALLWAY_STOPS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

impl<const SIZE: usize> Burrow<SIZE> {
    fn solved(&self) -> bool {
        [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D]
            .iter()
            .all(|&amp| self.rooms[amp as usize].iter().all(|&a| a == Some(amp)))
    }

    fn with_hallway_stop(mut self, amphipod: Amphipod, pos: usize) -> Self {
        self.hallway[pos] = Some(amphipod);
        self
    }

    fn with_room_stop(mut self, amphipod: Amphipod, dist: usize) -> (Self, usize) {
        let pos = (0..SIZE)
            .rev()
            .find_or_first(|p| self.rooms[amphipod as usize][*p].is_none())
            .unwrap();

        self.rooms[amphipod as usize][pos] = Some(amphipod);

        (self, (dist + pos + 1) * amphipod.cost())
    }

    fn room_free(&self, amphipod: Amphipod) -> bool {
        self.rooms[amphipod as usize]
            .iter()
            .all(|&r| r.map_or(true, |a| a == amphipod))
    }

    fn hallway_moves_from_room(
        &self,
        room: Amphipod,
        amphipod: Amphipod,
        dist: usize,
    ) -> Vec<(Self, usize)> {
        let pos = room.hallway_pos();
        let mut result = vec![];

        result.extend(
            (0..pos)
                .rev()
                .take_while(|&p| self.hallway[p].is_none())
                .filter(|p| HALLWAY_STOPS.contains(p))
                .map(|p| {
                    (
                        self.with_hallway_stop(amphipod, p),
                        (dist + pos - p) * amphipod.cost(),
                    )
                }),
        );

        result.extend(
            (pos + 1..=10)
                .take_while(|&p| self.hallway[p].is_none())
                .filter(|p| HALLWAY_STOPS.contains(p))
                .map(|p| {
                    (
                        self.with_hallway_stop(amphipod, p),
                        (dist + p - pos) * amphipod.cost(),
                    )
                }),
        );

        result
    }

    fn moves_from_hallway(&self) -> Vec<(Self, usize)> {
        let mut moves = vec![];

        for pos in HALLWAY_STOPS {
            if let Some(amphipod) = self.hallway[pos] {
                let room_pos = amphipod.hallway_pos();
                if self.room_free(amphipod) {
                    let mut range;
                    let dist;

                    if room_pos > pos {
                        range = pos + 1..=room_pos;
                        dist = room_pos - pos;
                    } else {
                        range = room_pos..=pos - 1;
                        dist = pos - room_pos;
                    };

                    if range.all(|p| self.hallway[p].is_none()) {
                        let mut next = self.clone();
                        next.hallway[pos] = None;
                        moves.push(next.with_room_stop(amphipod, dist));
                    }
                }
            }
        }

        moves
    }

    fn moves(&self) -> Vec<(Self, usize)> {
        let mut moves = vec![];

        for amphipod in [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D] {
            if self.room_free(amphipod) {
                continue;
            }

            let (pos, first) = self.rooms[amphipod as usize]
                .iter()
                .enumerate()
                .find(|(_, &r)| r.is_some())
                .unwrap();

            let mut next = *self;
            next.rooms[amphipod as usize][pos] = None;
            moves.append(&mut next.hallway_moves_from_room(amphipod, first.unwrap(), pos + 1))
        }

        moves.append(&mut self.moves_from_hallway());

        moves
    }

    fn approx_distance(&self) -> usize {
        let mut total = 0;

        for room in [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D] {
            for (pos, amphipod) in self.rooms[room as usize].iter().enumerate() {
                if let Some(amphipod) = amphipod {
                    total += ((pos + 1) + amphipod.hallway_pos().abs_diff(room.hallway_pos()))
                        * amphipod.cost();
                }
            }
        }

        for (pos, amphipod) in self.hallway.iter().enumerate() {
            if let Some(amphipod) = amphipod {
                total += amphipod.hallway_pos().abs_diff(pos) * amphipod.cost();
            }
        }

        total
    }
}

fn cell_to_string(amphipod: Option<Amphipod>) -> String {
    match amphipod {
        Some(amphipod) => format!("{}", amphipod),
        None => ".".to_string(),
    }
}

impl<const SIZE: usize> Display for Burrow<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for amphipod in self.hallway {
            write!(f, "{}", cell_to_string(amphipod))?;
        }

        write!(f, "\n")?;

        for i in 0..SIZE {
            writeln!(
                f,
                "  {} {} {} {}",
                cell_to_string(self.rooms[0][i]),
                cell_to_string(self.rooms[1][i]),
                cell_to_string(self.rooms[2][i]),
                cell_to_string(self.rooms[3][i])
            )?;
        }

        Ok(())
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let a0 = Amphipod::from_char(data[2].chars().nth(3).unwrap());
    let b0 = Amphipod::from_char(data[2].chars().nth(5).unwrap());
    let c0 = Amphipod::from_char(data[2].chars().nth(7).unwrap());
    let d0 = Amphipod::from_char(data[2].chars().nth(9).unwrap());
    let a1 = Amphipod::from_char(data[3].chars().nth(3).unwrap());
    let b1 = Amphipod::from_char(data[3].chars().nth(5).unwrap());
    let c1 = Amphipod::from_char(data[3].chars().nth(7).unwrap());
    let d1 = Amphipod::from_char(data[3].chars().nth(9).unwrap());

    let burrow = Burrow {
        hallway: [None; 11],
        rooms: [[a0, a1], [b0, b1], [c0, c1], [d0, d1]],
    };

    let path = astar(
        &burrow,
        |b| b.moves(),
        |b| b.approx_distance(),
        |b| b.solved(),
    )
    .unwrap();

    println!("[Part 1] {:?}", path.1);

    let burrow = Burrow {
        hallway: [None; 11],
        rooms: [
            [a0, Some(Amphipod::D), Some(Amphipod::D), a1],
            [b0, Some(Amphipod::C), Some(Amphipod::B), b1],
            [c0, Some(Amphipod::B), Some(Amphipod::A), c1],
            [d0, Some(Amphipod::A), Some(Amphipod::C), d1],
        ],
    };

    let path = astar(
        &burrow,
        |b| b.moves(),
        |b| b.approx_distance(),
        |b| b.solved(),
    )
    .unwrap();

    println!("[Part 2] {:?}", path.1);
}
