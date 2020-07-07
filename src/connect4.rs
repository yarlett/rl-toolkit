extern crate test;
use crate::board;

use rand::seq::SliceRandom;

pub struct Connect4 {
    board: board::Board,
    player: usize,
}

impl Connect4 {
    pub fn coded_action(&self) -> Vec<f32> {
        vec![0f32; self.board.get_j()]
    }

    pub fn coded_state(&self) -> Vec<f32> {
        let mut vec = vec![0f32; self.board.len()];
        for i in 0..self.board.len() {
            vec[i] = self.board.get(i) as f32
        }
        vec
    }

    pub fn get_moves(&self) -> Vec<usize> {
        let mut js = Vec::new();
        for j in 0..self.board.get_j() {
            if self.board.get_ij(0, j) == 0 {
                js.push(j);
            }
        }
        js
    }

    // Performs Monte Carlo Tree Search (MCTS) startwing with a fresh game, under a random policy, until the game terminates.
    pub fn mcts(&mut self) -> Vec<(Vec<f32>, Vec<f32>)> {
        let mut data = Vec::new();
        self.reset();
        // Make move until game is finished.
        loop {
            // Pick a move to play.
            let moves = self.get_moves();
            if moves.len() == 0 {
                break;
            }
            let mv = moves.choose(&mut rand::thread_rng()).unwrap();
            // Make the move.
            let (i, j) = self.play(*mv);
            // Update states_actions.
            let s = self.coded_state();
            let mut a = self.coded_action();
            a[*mv] = 1.0;
            data.push((s, a));
            //
            if self.winning_point(i, j) {
                break;
            }
        }
        data
    }

    pub fn new() -> Connect4 {
        Connect4 {
            board: board::Board::new(6, 7),
            player: 1,
        }
    }

    pub fn play(&mut self, j: usize) -> (usize, usize) {
        // Find the lowest empty point in the column.
        let mut i = 0;
        loop {
            if (self.board.get_ij(i, j) == 0)
                & ((i == (self.board.get_i() - 1)) || (self.board.get_ij(i + 1, j) != 0))
            {
                break;
            }
            i += 1;
        }
        // Put the current player's piece at this location and switch players.
        self.board.put_ij(i, j, self.player);
        self.switch_players();
        (i, j)
    }

    pub fn player(&self) -> usize {
        self.player
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.player = 1;
    }

    // Indicates whether specified point is part of a winning 4.
    pub fn winning_point(&self, i: usize, j: usize) -> bool {
        if self.board.longest_col(i, j) >= 4 {
            return true;
        }
        if self.board.longest_diag_1(i, j) >= 4 {
            return true;
        }
        if self.board.longest_diag_2(i, j) >= 4 {
            return true;
        }
        if self.board.longest_row(i, j) >= 4 {
            return true;
        }
        false
    }

    pub fn to_string(&self) -> String {
        self.board.to_string()
    }

    fn switch_players(&mut self) {
        if self.player == 1 {
            self.player = 2
        } else {
            self.player = 1
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[bench]
    fn mcts(b: &mut Bencher) {
        let mut game = Connect4::new();
        b.iter(|| game.mcts());
    }
}
