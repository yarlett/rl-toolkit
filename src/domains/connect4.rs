extern crate test;
use crate::domain::Domain;
use crate::domains::board::Board;

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
        for direction in &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            if self
                .board
                .run_length(i as isize, j as isize, direction.0, direction.1)
                >= 4
            {
                return true;
            }
        }
        false
    }
}

impl Domain for Connect4 {
    type Action = (usize, usize); // (player, column)
    type Agent = usize; // player
    type Reward = f32; // reward
    type State = Vec<usize>; // board state

    fn act(&mut self, action: Self::Action) -> Vec<(Self::Agent, Self::Reward)> {
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

    fn get_agents(&self) -> Vec<Self::Agent> {
        vec![1, 2]
    }

    fn get_state(&self) -> Self::State {
        self.board.get_board()
    }

    fn reward_add(&self, r1: &Self::Reward, r2: &Self::Reward) -> Self::Reward {
        r1 + r2
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
        let (data, rewards) = game.simulate();
        for i in 0..data.len() {
            println!("{:?} {:?}", data[i], rewards[i]);
        }
    }

    #[bench]
    fn simulate_bench(b: &mut Bencher) {
        let mut game = Connect4::new();
        b.iter(|| game.simulate());
    }
}
