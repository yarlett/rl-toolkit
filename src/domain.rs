use rand::seq::SliceRandom;

pub trait Domain {
    type Agent: Clone + Eq;
    type Action: Clone;
    type Reward: Clone + Default;
    type State;

    // Apply action to update state and returns optional reward.
    fn act(&mut self, _: Self::Action) -> Vec<(Self::Agent, Self::Reward)>;
    // Get actions that can be taken given the current state (returns None if no actions are possible and the game is finished).
    fn get_actions(&self) -> Vec<Self::Action>;
    // Returns all agents.
    fn get_agents(&self) -> Vec<Self::Agent>;
    // Returns the current state.
    fn get_state(&self) -> Self::State;
    // Add two rewards.
    fn reward_add(&self, _: &Self::Reward, _: &Self::Reward) -> Self::Reward;
    // Resets state to starting condition.
    fn reset(&mut self);

    // Starts a fresh game and performs Monte Carlo Tree Search (MCTS) under a random policy until the game terminates.
    fn simulate(
        &mut self,
    ) -> (
        Vec<(Self::State, Self::Action, Vec<(Self::Agent, Self::Reward)>)>,
        Vec<Vec<(Self::Agent, Self::Reward)>>,
    ) {
        // Reset state.
        self.reset();
        // Collect (S, A, R) triples by making random moves until game is finished.
        let agents = self.get_agents();
        let mut data = Vec::new();
        loop {
            // Get state.
            let state = self.get_state();
            // Get valid actions.
            let actions = self.get_actions();
            // If no actions returned then end the game.
            if actions.is_empty() {
                break;
            };
            // Choose a random action and apply to get rewards.
            let action = actions.choose(&mut rand::thread_rng()).unwrap();
            let rewards = self.act(action.to_owned());
            data.push((state, action.to_owned(), rewards));
        }
        // Initialize backpropagated rewards.
        let mut rewards: Vec<Vec<(Self::Agent, Self::Reward)>> = Vec::new();
        for _ in 0..data.len() {
            let mut r: Vec<(Self::Agent, Self::Reward)> = Vec::new();
            for a in &agents {
                r.push((a.to_owned(), Self::Reward::default()));
            }
            rewards.push(r);
        }
        // Backpropagate final rewards back to the start of the game.
        for t in (0..data.len()).rev() {
            // Push sparse rewards into dense rewards for current step.
            for (agent, reward) in &data[t].2 {
                for (agent2, reward2) in &mut rewards[t] {
                    if agent2 == agent {
                        *reward2 = self.reward_add(reward2, reward);
                    }
                }
            }
            // Copy rewards up from time t + 1 (if it exists).
            if t < (rewards.len() - 1) {
                for i in 0..rewards[t + 1].len() {
                    rewards[t][i].1 = self.reward_add(&rewards[t][i].1, &rewards[t + 1][i].1);
                }
            }
        }
        // Return data.
        (data, rewards)
    }
}
