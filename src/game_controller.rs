pub mod card_controller;
mod deck_controller;
mod diff_controller;
pub mod game_actions;
pub mod move_controller;
mod player;
pub mod player_choices;

use player::Player;

#[derive(Debug, Clone)]
pub struct FullGame {
    pub start: GameState,
    pub current: GameState,
    pub moves: Vec<move_controller::Move>,
}
impl FullGame {
    // pub fn start_new_game() -> Self {

    //     let start_state = GameState::new();

    //     FullGame {
    //         start: start_state.clone(),
    //         current: start_state,
    //         moves: Vec::new(),
    //     }
    // }
    pub fn start_empty_game(gs: &GameState) -> Self {
        let mut start_state = gs.clone();
        start_state.set_up();

        FullGame {
            start: start_state.clone(),
            current: start_state,
            moves: Vec::new(),
        }
    }
    pub fn start_mid_game(gs: &GameState) -> Self {
        FullGame {
            start: gs.clone(),
            current: gs.clone(),
            moves: Vec::new(),
        }
    }

    pub fn get_winner(&self) -> u8 {
        if !self.current.game_over() {
            panic!("no winner");
        }
        if self.current.first_player.keys.is_empty() {
            1
        } else {
            0
        }
    }

    pub fn play_game(&mut self) {
        if self.current.turn_number <= 0 {
            let choice = player_choices::mulligan(&self.current);
            self.current.add(&choice.to);
            self.moves.push(choice);
            self.current.turn_number += 1;
            Self::play_game(self)
        } else {
            self.current = Self::take_a_turn(&self.current, &mut self.moves);
            //println!("Game over!")
        }
    }

    fn take_a_turn(gs: &GameState, moves: &mut Vec<move_controller::Move>) -> GameState {
        let mut part = 0;
        let mut current = gs.clone();
        while !current.game_over() {
            part = part % 7;
            match part {
                0 => {
                    if current.turn_number == 1 {
                        current.first_player.hand_size = 6;
                    }
                    part += 1;
                } // Start Turn
                1 => {
                    let choice = player_choices::forges(&current, current.turn_number);
                    current.add(&choice.to);
                    moves.push(choice);
                    part += 1
                } // Forge a Key
                2 => {
                    let choice = player_choices::house(&current);
                    current.add(&choice.to);
                    moves.push(choice);
                    part += 1
                } // Choose a house
                3 => {
                    let choice = player_choices::play(&current);
                    current.add(&choice.to);

                    // TODO: Rule of 6
                    if choice.to.first_player.is_empty()
                        && choice.to.second_player.is_empty()
                        && choice.to.turn_number.is_none()
                    {
                        moves.push(choice);
                        part += 1;
                    } else if current.turn_number == 1 {
                        moves.push(choice);
                        moves.push(move_controller::Move {
                            by: String::from("end of first turn"),
                            to: diff_controller::StateDiff::new(),
                        });
                        part += 1;
                    } else {
                        moves.push(choice);
                    }
                } // Play, discard, and use cards of the chosen house
                4 => {
                    let choice = player_choices::ready_cards(&current);
                    current.add(&choice.to);
                    moves.push(choice);
                    part += 1;
                } // Ready cards
                5 => {
                    let choice = player_choices::draw(&current); // TODO: reset first turn hand size
                    current.add(&choice.to);
                    moves.push(choice);
                    part += 1;
                } // Draw cards
                6 => {
                    //println!("turn over");
                    let choice = player_choices::end(&current);
                    current.add(&choice.to);
                    moves.push(choice);
                    part += 1;
                } // End Turn
                _ => panic!("not a valid turn"),
            }
        }
        return current.clone();
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub first_player: Player,
    pub second_player: Player,

    pub turn_number: i64,
}
impl GameState {
    // pub fn new(first_player: Player, second_player: Player) -> Self {
    //     GameState {
    //         first_player: first_player,
    //         second_player: second_player,
    //         turn_number: -1,
    //     }
    // }

    pub fn empty_new() -> Self {
        GameState {
            first_player: Player::empty_new(1),
            second_player: Player::empty_new(2),
            turn_number: -1,
        }
    }

    pub fn get_player_by_number(&self, player_number: i64) -> &Player {
        if player_number % 2 == 0 {
            &self.second_player
        } else {
            &self.first_player
        }
    }

    pub fn get_turn_player(&self) -> &Player {
        if self.turn_number % 2 == 0 {
            &self.second_player
        } else {
            &self.first_player
        }
    }

    pub fn add(&mut self, sd: &diff_controller::StateDiff) {
        // TODO: maybe room for further speed ups here??
        self.first_player.add(&sd.first_player);
        self.second_player.add(&sd.second_player);
        self.turn_number = sd.turn_number.unwrap_or(self.turn_number);
    }

    pub fn set_up(&mut self) {
        self.first_player.set_up(Some(1));
        self.second_player.set_up(None);
        self.turn_number = -1;
    }

    pub fn game_over(&self) -> bool {
        self.first_player.keys.len() == 0 || self.second_player.keys.len() == 0
    }
}
