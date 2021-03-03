pub mod game_controller;
// mod minimaxer;
mod most_cards;
//use std::io::{stdin, stdout, Write};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    //let player_names: [&str; 2] = [&get_player_name(), &get_player_name()];
    //let deck_names: [&str; 2] = [&get_deck_name(), &get_deck_name()];

    // let player_names: [&str; 2] = ["K", "y"];
    // let deck_names: [&str; 2] = [
    //     "48e005bb-b83d-4e28-a63e-657ad2eaf760",
    //     "48e005bb-b83d-4e28-a63e-657ad2eaf760",
    // ];

    let mut first_player_wins = 0;
    let games = 10000;

    let raw_state = game_controller::GameState::empty_new();

    for i in 0..games {
        if i % 100 == 0 {
            println!("{} Games finished", i);
        }
        let mut fg = game_controller::FullGame::start_empty_game(&raw_state);
        fg.play_game();
        if fg.get_winner() == 1 {
            first_player_wins += 1;
        }
    }

    println!(
        "total time: {}.{} secs",
        now.elapsed().as_secs(),
        now.elapsed().subsec_nanos()
    );
    println!(
        "first player win %: {} out of {} games",
        first_player_wins, games
    );
    println!(
        "second player win %: {} out of {} games",
        (games - first_player_wins),
        games
    );
}

// // fn get_player_name() -> String {
// //     let mut s = String::new();
// //     print!("Please enter the first player's name: ");
// //     let _ = stdout().flush();
// //     stdin()
// //         .read_line(&mut s)
// //         .expect("Did not enter a correct string");

// //     return s.clone();
// // }

// // fn get_deck_name() -> String {
// //     let mut s = String::new();
// //     print!("Please enter the first player's deck id: ");
// //     let _ = stdout().flush();
// //     stdin()
// //         .read_line(&mut s)
// //         .expect("Did not enter a correct string");

// //     return s.clone();
// // }
