use rand::seq::SliceRandom;
use rltoolkit::Connect4;

fn main() {
    for _ in 0..1_000 {
        // Initialize board.
        let mut connect4 = Connect4::new();
        loop {
            // Pick a random column.
            let mvs = connect4.plays();
            if mvs.len() == 0 {
                println!("DRAW!");
                println!("{}", connect4.to_string());
                break;
            }
            let mv = mvs.choose(&mut rand::thread_rng()).unwrap();
            let (i, j) = connect4.play(*mv);
            if connect4.winning_point(i, j) {
                println!("WIN!");
                println!("{}", connect4.to_string());
                break;
            }
        }
    }
}
