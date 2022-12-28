use std::collections::HashMap;

const NB_ROUNDS: usize = 2020;
const NB_ROUNDS_2: usize = 30_000_000;

struct NumberGame {
    start: Vec<usize>,
    index: usize,
    last: usize,
    last_dist: usize,
    indexes: HashMap<usize, usize>,
}

impl NumberGame {
    fn new(start: Vec<usize>) -> Self {
        NumberGame {
            start,
            index: 0,
            last: 0,
            last_dist: 0,
            indexes: HashMap::new(),
        }
    }

    fn play_n_rounds(&mut self, n: usize) {
        for _ in 0..n {
            self.play_round();
            // println!("SPoken: {}", self.last);
        }
    }

    fn play_round(&mut self) {
        let i: usize = self.index;
        if i < self.start.len() {
            let v: usize = self.start[i];
            (self.last, self.last_dist) = (v, 0);
            self.indexes.insert(v, i);
        } else {
            let tell = self.last_dist;
            self.last = tell;
            let last_index = self.indexes.entry(tell).or_insert(i);
            self.last_dist = i - *last_index;
            *last_index = i;
        }
        self.index += 1;
    }

    fn last_spoken(&self) -> usize {
        self.last
    }
}

fn main() {
    let now = std::time::Instant::now();

    let start: Vec<usize> = vec![2, 15, 0, 9, 1, 20];
    let mut game: NumberGame = NumberGame::new(start.clone());
    game.play_n_rounds(NB_ROUNDS);
    println!(
        "The {}th number to be spoken is: {}",
        NB_ROUNDS,
        game.last_spoken()
    );

    let mut game: NumberGame = NumberGame::new(start);
    game.play_n_rounds(NB_ROUNDS_2);
    println!(
        "The {}th number to be spoken is: {}",
        NB_ROUNDS_2,
        game.last_spoken()
    );

    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let start: Vec<usize> = vec![0, 3, 6];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 436);
    }
    #[test]
    fn part_1_test_2() {
        let start: Vec<usize> = vec![1, 3, 2];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 1);
    }
    #[test]
    fn part_1_test_3() {
        let start: Vec<usize> = vec![2, 1, 3];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 10);
    }
    #[test]
    fn part_1_test_4() {
        let start: Vec<usize> = vec![1, 2, 3];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 27);
    }
    #[test]
    fn part_1_test_5() {
        let start: Vec<usize> = vec![2, 3, 1];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 78);
    }
    #[test]
    fn part_1_test_6() {
        let start: Vec<usize> = vec![3, 2, 1];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 438);
    }
    #[test]
    fn part_1_test_7() {
        let start: Vec<usize> = vec![3, 1, 2];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS);
        assert_eq!(game.last_spoken(), 1836);
    }

    #[test]
    fn part_2_test_1() {
        let start: Vec<usize> = vec![0, 3, 6];
        let mut game: NumberGame = NumberGame::new(start);
        game.play_n_rounds(NB_ROUNDS_2);
        assert_eq!(game.last_spoken(), 175594);
    }
}
