use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn mod10(n: i16) -> i16 {
    (n - 1) % 10 + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pawns: [i16; 2],
    scores: [i16; 2],
}

const DIRAC_COUNTS: [(i16, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

impl State {
    fn initial(p1: i16, p2: i16) -> Self {
        Self {
            pawns: [p1, p2],
            scores: [0, 0],
        }
    }

    fn winner(&self) -> Option<usize> {
        if self.scores[0] >= 21 {
            Some(0)
        } else if self.scores[1] >= 21 {
            Some(1)
        } else {
            None
        }
    }

    fn evolve_deterministic(mut self, player: usize, die: i16) -> Self {
        self.pawns[player] = mod10(self.pawns[player] + 3 * die + 3);
        self.scores[player] += self.pawns[player];
        self
    }

    fn evolve_dirac(self, player: usize) -> impl Iterator<Item = (Self, u64)> {
        DIRAC_COUNTS.iter().map(move |&(roll, count)| {
            let mut state = self;
            state.pawns[player] = mod10(state.pawns[player] + roll);
            state.scores[player] += state.pawns[player];
            (state, count)
        })
    }
}

fn deterministic(p1: i16, p2: i16) -> i64 {
    let mut rolls = 0;
    let mut die = 1;
    let mut player = 1;
    let mut state = State::initial(p1, p2);

    while state.scores[player] <= 1000 {
        player = 1 - player;
        state = state.evolve_deterministic(player, die);
        rolls += 3;
        die = (die + 2) % 100 + 1;
    }

    state.scores[1 - player] as i64 * rolls
}

fn dirac(p1: i16, p2: i16) -> u64 {
    let mut states = HashMap::new();
    let mut player = 0;
    let mut wins = [0, 0];

    states.insert(State::initial(p1, p2), 1u64);

    while !states.is_empty() {
        let mut next_states = HashMap::new();

        for (state, count) in states {
            for (next_state, next_count) in state.evolve_dirac(player) {
                match next_state.winner() {
                    None => *next_states.entry(next_state).or_insert(0) += count * next_count,
                    Some(player) => wins[player] += count * next_count,
                }
            }
        }

        states = next_states;
        player = 1 - player;
    }

    wins[0].max(wins[1])
}

pub fn solve() {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let p1 = data[0].split(": ").nth(1).unwrap().parse().unwrap();
    let p2 = data[1].split(": ").nth(1).unwrap().parse().unwrap();

    println!("[Part 1] {:?}", deterministic(p1, p2));
    println!("[Part 2] {:?}", dirac(p1, p2));
}
