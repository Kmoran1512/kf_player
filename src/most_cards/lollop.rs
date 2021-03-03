/*
* NON-CHEATING
* This brain is built with the understanding that it is playing with a deck of only pips.
* for that reason it will never reap or make decisions about cards on the board
*/

pub fn chooser_switcher(
    moves: &Vec<super::super::game_controller::move_controller::Move>,
    gs: &super::super::game_controller::GameState,
    phase: &str,
) -> usize {
    match phase {
        "Key" => chooser_key(moves),
        "Mulligan" => chooser_mulligan(moves, gs),
        "House" => chooser_house(moves, gs),
        "Play" => chooser_plays(moves, gs),
        _ => panic!("no"),
    }
}

fn chooser_key(moves: &Vec<super::super::game_controller::move_controller::Move>) -> usize {
    moves.len() - 1
}

fn chooser_mulligan(
    _moves: &Vec<super::super::game_controller::move_controller::Move>,
    gs: &super::super::game_controller::GameState,
) -> usize {
    let hand_house_counts = get_location_house_counts(
        &gs.get_turn_player().deck.houses,
        &gs.get_turn_player().hand,
    );

    let max_count = hand_house_counts.iter().max().unwrap();
    let min_count = hand_house_counts.iter().min().unwrap();

    if 1 >= max_count - min_count {
        return 1; // This is a mulligan
    }

    0
}

fn chooser_house(
    _moves: &Vec<super::super::game_controller::move_controller::Move>,
    gs: &super::super::game_controller::GameState,
) -> usize {
    let turn_player = gs.get_turn_player();

    let hand_house_counts = get_location_house_counts(&turn_player.deck.houses, &turn_player.hand);

    let most_of_a_house = hand_house_counts.iter().max().unwrap();

    let mut location_of_highest_houses: Vec<usize> = Vec::new();
    for (i, house_count) in hand_house_counts.iter().enumerate() {
        if house_count == most_of_a_house {
            location_of_highest_houses.push(i);
        }
    }

    if location_of_highest_houses.len() > 1 {
        return tie_break_house_choice(
            &turn_player.deck.houses,
            &location_of_highest_houses,
            &turn_player.library,
        );
    }

    hand_house_counts
        .iter()
        .position(|r| r == most_of_a_house)
        .unwrap()
}

fn chooser_plays(
    moves: &Vec<super::super::game_controller::move_controller::Move>,
    gs: &super::super::game_controller::GameState,
) -> usize {
    let mut best_turn = 0;
    let mut i_best_turn: usize = 0;

    for (i, a_move) in moves.iter().enumerate() {
        let on_player_amber_diff = a_move
            .to
            .get_player_by_number(gs.turn_number)
            .amber
            .unwrap_or(gs.get_player_by_number(gs.turn_number).amber)
            - gs.get_player_by_number(gs.turn_number).amber;
        let off_player_amber_diff = a_move
            .to
            .get_player_by_number(gs.turn_number + 1)
            .amber
            .unwrap_or(gs.get_player_by_number(gs.turn_number + 1).amber)
            - gs.get_player_by_number(gs.turn_number + 1).amber;

        let amber_diff: i64 = on_player_amber_diff - off_player_amber_diff;
        if amber_diff > best_turn {
            best_turn = amber_diff;
            i_best_turn = i;
        }
    }

    i_best_turn
}

fn get_location_house_counts(
    houses: &[String; 3],
    location: &Vec<super::super::game_controller::card_controller::Card>,
) -> [u8; 3] {
    let mut count_array: [u8; 3] = [0, 0, 0];

    for card in location {
        if card.house == houses[0] {
            count_array[0] += 1
        } else if card.house == houses[1] {
            count_array[1] += 1
        } else if card.house == houses[2] {
            count_array[2] += 1
        }
    }

    count_array
}
fn tie_break_house_choice(
    houses: &[String; 3],
    location_of_tied_houses: &Vec<usize>,
    deck: &Vec<super::super::game_controller::card_controller::Card>,
) -> usize {
    let deck_house_counts = get_location_house_counts(houses, deck);

    let mut min: u8 = 200;
    let mut i_min: usize = 0;

    for (i, &h_i) in deck_house_counts.iter().enumerate() {
        if h_i < min && !location_of_tied_houses.iter().find(|&&h| h == i).is_none() {
            min = h_i;
            i_min = i;
        }
    }

    return i_min;
}
