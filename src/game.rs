use rand::seq::SliceRandom;

// Making this trait generic.
// A: Action
// S: State

pub trait Game {
    type Action;
    type Reward;

    fn act(&mut self, _: &Self::Action) -> Option<Self::Reward>;
    fn get_actions(&self) -> Vec<Self::Action>;
    fn coded_state(&self) -> Vec<f32>;
    fn coded_action(&self) -> Vec<f32>;
    fn reset(&mut self);
    // fn winning_point(&self, _: usize, _: usize) -> bool;

    // Starts a fresh game and performs Monte Carlo Tree Search (MCTS) under a random policy until the game terminates.
    fn mcts(&mut self) -> Vec<(Vec<f32>, Vec<f32>)> {
        self.reset();
        // Make move until game is finished.
        let data = Vec::new();
        loop {
            // Get valid actions and pick one randomly.
            let actions = self.get_actions();
            if actions.len() == 0 {
                break;
            }
            let action = actions.choose(&mut rand::thread_rng()).unwrap();
            // Take the action and get the reward.
            let _reward = self.act(action);

            // // Update states_actions.
            // let s = self.coded_state();
            // let a = self.coded_action();
            // // a[*action] = 1.0;
            // data.push((s, a));
            // //
            // if self.winning_point(i, j) {
            //     break;
            // }
        }
        data
    }
    //     // Derive the value vectors for the game by back-propagating value of final state to start of game.
    //     for (s, a) in &states_actions {
    //         println!("{:?} {:?}", s, a);
    //     }
    // }
}
