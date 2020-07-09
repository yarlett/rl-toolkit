use rand::seq::SliceRandom;

pub trait Game {
    type Action: Clone;
    type Reward;
    type State;

    // Apply action to update state and returns optional reward.
    fn act(&mut self, _: Self::Action) -> Option<Self::Reward>;
    // Get actions that can be taken given the current state.
    fn get_actions(&self) -> Option<Vec<Self::Action>>;
    fn get_state(&self) -> Self::State;
    // Resets state to starting condition.
    fn reset(&mut self);

    // Starts a fresh game and performs Monte Carlo Tree Search (MCTS) under a random policy until the game terminates.
    fn simulate(&mut self) -> Vec<(Self::State, Self::Action, Option<Self::Reward>)> {
        // Reset state.
        self.reset();
        // Collect (S, A, R) triples by making random moves until game is finished.
        let mut data = Vec::new();
        loop {
            // Get state.
            let state = self.get_state();
            // Get valid actions.
            let actions = self.get_actions();
            // Pick an action randomly.
            match actions {
                None => break,
                Some(actions_inner) => {
                    let action = actions_inner.choose(&mut rand::thread_rng()).unwrap();
                    let reward = self.act(action.to_owned());
                    data.push((state, action.to_owned(), reward));
                }
            }
        }
        // Backpropagate objective rewards back to the start of the game.

        // Return data.
        data
    }
}
