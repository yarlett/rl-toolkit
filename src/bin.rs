use rltoolkit::connect4::Connect4;

fn main() {
    let mut c4 = Connect4::new();
    for _ in 0..1 {
        // Initialize game and.
        for _ in 0..100 {
            c4.reset();
            let _game = c4.mcts();
        }
    }
}
//     // Derive the value vectors for the game by back-propagating value of final state to start of game.
//     for (s, a) in &states_actions {
//         println!("{:?} {:?}", s, a);
//     }
// }
