extern crate test;
use crate::board::Board;
use crate::game::Game;

#[derive(Clone)]
pub struct Connect4 {
    board: Board,
    player: usize,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4 {
            board: Board::new(6, 7),
            player: 1,
        }
    }

    pub fn player(&self) -> usize {
        self.player
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

    // Indicates whether specified point is part of a winning 4.
    fn winning_point(&self, i: usize, j: usize) -> bool {
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
}

impl Game for Connect4 {
    type Action = (usize, usize);  // (player, action)
    type Reward = (usize, f32);    // (player, reward)
    type State = Vec<usize>;       // board

    fn act(&mut self, action: Self::Action) -> Option<Self::Reward> {
        let (player, column) = action;
        // Find the lowest empty point in the column.
        let mut i = 0;
        loop {
            if (self.board.get_ij(i, column) == 0)
                & ((i == (self.board.get_i() - 1)) || (self.board.get_ij(i + 1, column) != 0))
            {
                break;
            }
            i += 1;
        }
        // Put the current player's piece at this location and switch players.
        self.board.put_ij(i, column, player);
        self.switch_players();
        // Determine reward.
        if self.winning_point(i, column) {
            return Some((player, 1.0));
        } else {
            return None;
        }
    }

    fn get_actions(&self) -> Option<Vec<Self::Action>> {
        let mut actions = Vec::new();
        let cols = self.board.get_j();
        let rows = self.board.get_i();
        // Iterate over columns.
        for col in 0..cols {
            // Find the row of the highest piece in the column.
            let mut row = 0;
            loop {
                if self.board.get_ij(row, col) != 0 {
                    break;
                }
                row += 1;
                if row >= rows {
                    break;
                }
            }
            // If the highest piece in the column is part of a winning position then no actions are possible.
            if (row < rows) {
                if self.winning_point(row, col) { return None; }
            }
            // If the row of the highest piece is > 0 then there's room to move in the row above.
            if (row > 0) {
                actions.push((self.player, col));
            }
        }
        // Return available actions if any.
        match actions.len() {
            0 => None,
            _ => Some(actions),
        }
    }

    fn get_state(&self) -> Self::State {
        self.board.get_board()
    }

    // fn coded_action(&self) -> Vec<f32> {
    //     vec![0f32; self.board.get_j()]
    // }
    //
    // fn coded_state(&self) -> Vec<f32> {
    //     let mut vec = vec![0f32; self.board.len()];
    //     for i in 0..self.board.len() {
    //         vec[i] = self.board.get(i) as f32
    //     }
    //     vec
    // }

    fn reset(&mut self) {
        self.board.reset();
        self.player = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn simulate_test() {
        let mut game = Connect4::new();
        let data = game.simulate();
        for d in &data {
            println!("{:?}", d);
        }
    }

    #[bench]
    fn simulate_bench(b: &mut Bencher) {
        let mut game = Connect4::new();
        b.iter(|| game.simulate());
    }
}
