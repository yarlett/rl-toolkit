use rand::seq::SliceRandom;
use rltoolkit::connect4::Connect4;

fn main() {
    for _ in 0..1 {
        // Initialize game and.
        let mut c4 = Connect4::new();
        let mut m = 0;

        let mut states_actions = Vec::new();
        states_actions.push((c4.coded_state(), c4.coded_action()));
        // Make random moves until game terminates.
        loop {
            // Pick a random column to play.
            let mvs = c4.plays();
            if mvs.len() == 0 {
                println!("Move {} DRAW!", m);
                println!("{}", c4.to_string());
                break;
            }
            let mv = mvs.choose(&mut rand::thread_rng()).unwrap();
            // Make the move.
            let (i, j) = c4.play(*mv);
            // Update states_actions.
            let s = c4.coded_state();
            let mut a = c4.coded_action();
            a[*mv] = 1.0;
            states_actions.push((s, a));
            //
            println!("Move {}", m);
            println!("{}", c4.to_string());
            if c4.winning_point(i, j) {
                println!("Move {} WIN!", m);
                println!("{}", c4.to_string());
                break;
            }
            m += 1;
        }
        // Derive the value vectors for the game by back-propagating value of final state to start of game.
        for (s, a) in &states_actions {
            println!("{:?} {:?}", s, a);
        }
    }
}
