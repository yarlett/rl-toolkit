use rand::seq::SliceRandom;

pub trait Game {
    fn get_moves(&self) -> Vec<usize>;
    fn coded_state(&self) -> Vec<f32>;
    fn coded_action(&self) -> Vec<f32>;
    // fn new() -> Self;
    fn play(&mut self, _: usize) -> (usize, usize);
    fn reset(&mut self);
    fn winning_point(&self, _: usize, _: usize) -> bool;

    // Starts a fresh game and performs Monte Carlo Tree Search (MCTS) under a random policy until the game terminates.
    fn mcts(&mut self) -> Vec<(Vec<f32>, Vec<f32>)> {
        self.reset();
        // Make move until game is finished.
        let mut data = Vec::new();
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
    //     // Derive the value vectors for the game by back-propagating value of final state to start of game.
    //     for (s, a) in &states_actions {
    //         println!("{:?} {:?}", s, a);
    //     }
    // }
}
