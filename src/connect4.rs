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
    type Action = usize;
    type Reward = f32;
    type State = (Vec<usize>, usize);

    fn act(&mut self, j: Self::Action) -> Option<Self::Reward> {
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
        // Determine reward.
        if self.winning_point(i, j) {
            return Some(1.0);
        } else {
            return None;
        }
    }

    fn get_actions(&self) -> Option<Vec<Self::Action>> {
        let mut js = Vec::new();
        for j in 0..self.board.get_j() {
            if self.board.get_ij(0, j) == 0 {
                js.push(j);
            }
        }
        match js.len() {
            0 => None,
            _ => Some(js),
        }
    }

    fn get_state(&self) -> Self::State {
        (self.board.get_board(), self.player)
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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[bench]
    fn simulate(b: &mut Bencher) {
        let mut game = Connect4::new();
        b.iter(|| game.simulate());
    }
}
