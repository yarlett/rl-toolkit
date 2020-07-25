use rltoolkit::domain::Domain;
use rltoolkit::domains::connect4::Connect4;

fn main() {
    let mut c4 = Connect4::new();
    for _ in 0..1 {
        // Initialize game and.
        for _ in 0..100 {
            c4.reset();
            let _game = c4.simulate();
        }
    }
}
