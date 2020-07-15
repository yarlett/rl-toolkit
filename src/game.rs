use rand::seq::SliceRandom;

pub trait Game {
    type Action: Clone;
    type Reward;
    type State;

    // Apply action to update state and returns optional reward.
    fn act(&mut self, _: Self::Action) -> Vec<Self::Reward>;
    // Get actions that can be taken given the current state (returns None if no actions are possible and the game is finished).
    fn get_actions(&self) -> Vec<Self::Action>;
    // Returns the current state.
    fn get_state(&self) -> Self::State;
    // Resets state to starting condition.
    fn reset(&mut self);

    // Starts a fresh game and performs Monte Carlo Tree Search (MCTS) under a random policy until the game terminates.
    fn simulate(&mut self) -> Vec<(Self::State, Self::Action, Vec<Self::Reward>)> {
        // Reset state.
        self.reset();
        // Collect (S, A, R) triples by making random moves until game is finished.
        let mut data = Vec::new();
        loop {
            // Get state.
            let state = self.get_state();
            // Get valid actions.
            let actions = self.get_actions();
            // If no actions returned then end the game.
            if actions.len() == 0 {
                break;
            };
            // Choose a random action and apply to get rewards.
            let action = actions.choose(&mut rand::thread_rng()).unwrap();
            let rewards = self.act(action.to_owned());
            data.push((state, action.to_owned(), rewards));
        }
        // Backpropagate objective rewards back to the start of the game.

        // Return data.
        data
    }
}

// struct GameTurn {
//     actions: Vec<Action>,
//     state: State,
//     rewards: Vec<Reward>,
// }
