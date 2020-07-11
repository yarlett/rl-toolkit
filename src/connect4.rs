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
        self.player += 1;
        if self.player == 3 {
            self.player = 1;
        };
    }

    // Indicates whether specified point is part of a winning 4.
    fn winning_point(&self, i: usize, j: usize) -> bool {
        if self.board.get_ij(i, j) == 0 {
            return false;
        };

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
    type Action = (usize, usize); // (player, column)
    type Reward = (usize, f32); // (player, reward)
    type State = Vec<usize>; // board

    fn act(&mut self, action: Self::Action) -> Vec<Self::Reward> {
        let (player, column) = action;
        let mut rewards = Vec::new();
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
        let other_player = if player == 1 { 2 } else { 1 };
        if self.winning_point(i, column) {
            rewards.push((player, 1.0));
            rewards.push((other_player, -1.0));
        }
        rewards
    }

    fn get_actions(&self) -> Vec<Self::Action> {
        let mut actions = Vec::new();
        let ni = self.board.get_i();
        let nj = self.board.get_j();
        // If any point is a winning point then return no actions because game is finished.
        for i in 0..ni {
            for j in 0..nj {
                if self.winning_point(i, j) {
                    return actions;
                };
            }
        }
        // Iterate over columns and if there's space then add action that involves placing a piece in that column.
        for j in 0..nj {
            if self.board.get_ij(0, j) == 0 {
                actions.push((self.player, j));
            }
        }
        // Return available actions if any.
        actions
    }

    fn get_state(&self) -> Self::State {
        self.board.get_board()
    }

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
