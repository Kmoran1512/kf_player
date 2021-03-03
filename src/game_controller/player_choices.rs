use super::card_controller::Card;
use super::diff_controller::{PlayerDiff, StateDiff};
use super::move_controller::{Minds, Move};

pub fn mulligan(gs: &super::GameState) -> Move {
    let mut mulligan_state: StateDiff = StateDiff::new();
    let mut mulligan_player: PlayerDiff = PlayerDiff::new();

    let mulligan_player_ref = gs.get_turn_player();

    let set_set_hand_size = mulligan_player_ref.hand.len() as i64;

    let (temp_library, temp_discard, temp_hand) = PlayerDiff::reset_deck(
        &mulligan_player_ref.library,
        &mulligan_player_ref.discard,
        &mulligan_player_ref.hand,
    );

    let (temp_library, temp_discard, temp_hand, _) = PlayerDiff::draw_to_size(
        &temp_library,
        &temp_discard,
        &temp_hand,
        &(set_set_hand_size - 1),
        &0,
    );

    mulligan_player.library = Some(temp_library);
    mulligan_player.discard = Some(temp_discard);
    mulligan_player.hand = Some(temp_hand);

    mulligan_state.insert_playerdiff_by_number(mulligan_player, gs.turn_number);

    let decisions = vec![
        Move {
            to: StateDiff::new(),
            by: "No Mulligan".to_string(),
        },
        Move {
            to: mulligan_state,
            by: "Taking Mulligan".to_string(),
        },
    ];

    decisions[Minds::choose(
        &gs.get_turn_player().brain,
        &decisions,
        &gs,
        Some("Mulligan"),
    )]
    .clone()
}

pub fn forges(gs: &super::GameState, player_number_to_forge: i64) -> Move {
    let player_to_forge = gs.get_player_by_number(player_number_to_forge);

    let total_amber = player_to_forge.amber; // TODO: Safeplace rule

    if total_amber < gs.get_player_by_number(player_number_to_forge).key_cost {
        return Move {
            to: StateDiff::new(),
            by: "Not enough amber to forge".to_string(),
        };
    }

    // print!("A Key!");

    let mut decisions: Vec<Move> = Vec::new();

    for key in player_to_forge.keys.iter() {
        let mut ret_pd = PlayerDiff::new();
        let mut key_vec = player_to_forge.keys.clone();

        key_vec.remove(
            key_vec
                .iter()
                .position(|x| &x == &key)
                .expect("key not found"),
        );

        ret_pd.amber = Some(player_to_forge.amber - player_to_forge.key_cost);
        ret_pd.keys = Some(key_vec);

        let mut ret_sd = StateDiff::new();
        ret_sd.insert_playerdiff_by_number(ret_pd, player_number_to_forge);

        decisions.push(Move {
            by: format!("forging the {} key", key),
            to: ret_sd,
        });
    }

    decisions[super::move_controller::Minds::choose(
        &gs.get_turn_player().brain,
        &decisions,
        &gs,
        Some("Key"),
    )]
    .clone()
}

pub fn house(gs: &super::GameState) -> Move {
    let player_to_choose = gs.get_turn_player();

    if !player_to_choose.active_house.is_none() {
        return Move {
            to: StateDiff::new(),
            by: format!(
                "Must choose {}",
                player_to_choose.active_house.clone().unwrap()
            ),
        };
    }

    // TODO: Restringuntis
    let mut decisions: Vec<Move> = Vec::new();

    for house in player_to_choose.deck.houses.iter() {
        let mut ret_pd = PlayerDiff::new();
        ret_pd.active_house = Some(Some(house.clone()));

        let mut ret_sd = StateDiff::new();
        ret_sd.insert_playerdiff_by_number(ret_pd, gs.get_turn_player().player_number);

        decisions.push(Move {
            by: format!("will play in house {}", house),
            to: ret_sd,
        });
    }

    decisions[super::move_controller::Minds::choose(
        &gs.get_turn_player().brain,
        &decisions,
        &gs,
        Some("House"),
    )]
    .clone()
}

pub fn ready_cards(gs: &super::GameState) -> Move {
    let mut ret_pd = PlayerDiff::new();

    let mut temp_creature_board: Vec<Card> = Vec::new();
    let mut temp_artifact_board: Vec<Card> = Vec::new();

    for artifact in &gs.get_turn_player().artifact_board {
        temp_artifact_board.push(super::card_controller::card_list::ready(artifact));
    }
    for creature in &gs.get_turn_player().creature_board {
        temp_creature_board.push(super::card_controller::card_list::ready(creature));
    }

    ret_pd.creature_board = Some(temp_creature_board);
    ret_pd.artifact_board = Some(temp_artifact_board);

    let mut ret_sd = StateDiff::new();
    ret_sd.insert_playerdiff_by_number(ret_pd, gs.get_turn_player().player_number);

    Move {
        by: String::from("Readies Cards"),
        to: ret_sd,
    }
}

pub fn draw(gs: &super::GameState) -> Move {
    let drawing_player = gs.get_turn_player();
    let mut ret_pd = PlayerDiff::new();

    let (temp_library, temp_discard, temp_hand, temp_chains) = PlayerDiff::draw_to_size(
        &drawing_player.library,
        &drawing_player.discard,
        &drawing_player.hand,
        &drawing_player.hand_size,
        &drawing_player.chains,
    );

    ret_pd.library = Some(temp_library);
    ret_pd.discard = Some(temp_discard);
    ret_pd.hand = Some(temp_hand);
    ret_pd.chains = Some(temp_chains);

    let mut ret_sd = StateDiff::new();
    ret_sd.insert_playerdiff_by_number(ret_pd, gs.get_turn_player().player_number);

    Move {
        by: String::from("Draws cards for end of turn"),
        to: ret_sd,
    }
}

pub fn end(gs: &super::GameState) -> Move {
    // TODO: Card effects that trigger on EOT
    let mut ret_sd = StateDiff::new();
    let mut ret_pd = PlayerDiff::new();

    ret_pd.active_house = Some(None);
    ret_sd.insert_playerdiff_by_number(ret_pd, gs.get_turn_player().player_number);

    ret_sd.turn_number = Some(gs.turn_number + 1);
    Move {
        by: String::from("end of turn"),
        to: ret_sd,
    }
}

pub fn play(gs: &super::GameState) -> Move {
    let mut decisions: Vec<Move> = Vec::new();
    decisions.push(Move {
        by: String::from("end of turn"),
        to: StateDiff::new(),
    });

    // TODO: Play cards
    // TODO: Discard
    let hand = &gs.get_turn_player().hand;
    let active_house = gs.get_turn_player().active_house.as_ref().unwrap();

    for card in hand {
        let card_house = &card.hard_info.house;
        if card_house == active_house {
            decisions.push(Move {
                by: format!["Plays {}", card.hard_info.name],
                to: super::card_controller::card_list::play(card, gs),
            });
            decisions.push(Move {
                by: format!["Plays {}", card.hard_info.name],
                to: super::card_controller::card_list::discard(card, gs),
            });
        }
    }
    //? Use
    // TODO: Reap
    // TODO: Fight
    // TODO: Action / Omni

    decisions[super::move_controller::Minds::choose(
        &gs.get_turn_player().brain,
        &decisions,
        &gs,
        Some("Play"),
    )]
    .clone()
}
