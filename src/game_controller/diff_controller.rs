use rand::seq::SliceRandom;
use rand::thread_rng;

use super::player::Player;

use super::card_controller::Card;

#[derive(Debug, Clone)]
pub struct StateDiff {
    pub first_player: PlayerDiff,
    pub second_player: PlayerDiff,

    pub turn_number: Option<i64>,
}
impl StateDiff {
    pub fn new() -> Self {
        Self {
            first_player: PlayerDiff::new(),
            second_player: PlayerDiff::new(),
            turn_number: None,
        }
    }
    pub fn set_equal(gs: &super::GameState) -> Self {
        Self {
            first_player: PlayerDiff::set_equal(&gs.first_player),
            second_player: PlayerDiff::set_equal(&gs.second_player),
            turn_number: Some(gs.turn_number),
        }
    }
    pub fn subtract(old_gs: &super::GameState, new_gs: &super::GameState) -> Self {
        Self {
            first_player: PlayerDiff::subtract(&old_gs.first_player, &new_gs.first_player),
            second_player: PlayerDiff::subtract(&old_gs.second_player, &new_gs.second_player),
            turn_number: if old_gs.turn_number != new_gs.turn_number {
                Some(new_gs.turn_number)
            } else {
                None
            },
        }
    }

    pub fn insert_playerdiff_by_number(&mut self, pd: PlayerDiff, num: i64) {
        if num % 2 == 0 {
            self.second_player = pd;
        } else {
            self.first_player = pd;
        }
    }

    pub fn get_player_by_number(&self, num: i64) -> &PlayerDiff {
        if num % 2 == 0 {
            &self.second_player
        } else {
            &self.first_player
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDiff {
    pub creature_board: Option<Vec<Card>>,
    pub artifact_board: Option<Vec<Card>>,

    pub hand: Option<Vec<Card>>,
    pub archives: Option<Vec<Card>>,
    pub library: Option<Vec<Card>>,
    pub discard: Option<Vec<Card>>,
    pub purged: Option<Vec<Card>>,

    pub amber: Option<i64>,
    pub keys: Option<Vec<&'static str>>,
    pub chains: Option<i64>,

    pub key_cost: Option<i64>,
    pub hand_size: Option<i64>,

    pub active_house: Option<Option<String>>,
}
impl PlayerDiff {
    pub fn new() -> Self {
        Self {
            creature_board: None,
            artifact_board: None,
            hand: None,
            archives: None,
            library: None,
            discard: None,
            purged: None,
            amber: None,
            keys: None,
            chains: None,
            key_cost: None,
            hand_size: None,
            active_house: None,
        }
    }
    pub fn set_equal(player: &Player) -> Self {
        Self {
            creature_board: Some(player.creature_board.clone()),
            artifact_board: Some(player.artifact_board.clone()),
            hand: Some(player.hand.clone()),
            archives: Some(player.archives.clone()),
            library: Some(player.library.clone()),
            discard: Some(player.discard.clone()),
            purged: Some(player.purged.clone()),
            amber: Some(player.amber),
            keys: Some(player.keys.clone()),
            chains: Some(player.chains),
            key_cost: Some(player.key_cost),
            hand_size: Some(player.hand_size),
            active_house: Some(player.active_house.clone()),
        }
    }
    pub fn subtract(old_player: &Player, new_player: &Player) -> Self {
        Self {
            creature_board: Some(new_player.creature_board.clone()),
            artifact_board: Some(new_player.artifact_board.clone()),
            hand: if old_player.hand != new_player.hand {
                Some(new_player.hand.clone())
            } else {
                None
            },
            archives: if old_player.archives != new_player.archives {
                Some(new_player.archives.clone())
            } else {
                None
            },
            library: if old_player.library != new_player.library {
                Some(new_player.library.clone())
            } else {
                None
            },
            discard: if old_player.discard != new_player.discard {
                Some(new_player.discard.clone())
            } else {
                None
            },
            purged: if old_player.purged != new_player.purged {
                Some(new_player.purged.clone())
            } else {
                None
            },
            amber: if old_player.amber != new_player.amber {
                Some(new_player.amber)
            } else {
                None
            },
            keys: if old_player.keys != new_player.keys {
                Some(new_player.keys.clone())
            } else {
                None
            },
            chains: if old_player.chains != new_player.chains {
                Some(new_player.chains)
            } else {
                None
            },
            key_cost: if old_player.key_cost != new_player.key_cost {
                Some(new_player.key_cost)
            } else {
                None
            },
            hand_size: if old_player.hand_size != new_player.hand_size {
                Some(new_player.hand_size)
            } else {
                None
            },
            active_house: if old_player.active_house != new_player.active_house {
                Some(new_player.active_house.clone())
            } else {
                None
            },
        }
    }

    pub fn shuffle_deck(library: &Vec<Card>) -> Vec<Card> {
        let mut temp_library = library.clone();
        temp_library.shuffle(&mut thread_rng());
        temp_library
    }
    pub fn reshuffle_deck(library: &Vec<Card>, discard: &Vec<Card>) -> (Vec<Card>, Vec<Card>) {
        let mut temp_library = library.clone();
        temp_library.append(&mut discard.clone());
        temp_library.shuffle(&mut thread_rng());
        (temp_library, Vec::new())
    }
    pub fn reset_deck(
        library: &Vec<Card>,
        discard: &Vec<Card>,
        hand: &Vec<Card>,
    ) -> (Vec<Card>, Vec<Card>, Vec<Card>) {
        let mut temp_library = library.clone();
        temp_library.append(&mut hand.clone());
        temp_library.append(&mut discard.clone());
        temp_library.shuffle(&mut thread_rng());
        (temp_library, Vec::new(), Vec::new())
    }

    pub fn draw(
        library: &Vec<Card>,
        discard: &Vec<Card>,
        hand: &Vec<Card>,
    ) -> (Vec<Card>, Vec<Card>, Vec<Card>) {
        if library.is_empty() {
            if discard.is_empty() {
                return (library.clone(), discard.clone(), hand.clone());
            }
            let (new_library, new_discard) = &Self::reshuffle_deck(library, discard);
            return Self::draw(new_library, new_discard, hand);
        }

        let (mut ret_library, ret_discard, mut ret_hand) =
            (library.clone(), discard.clone(), hand.clone());
        ret_hand.push(ret_library.pop().unwrap());
        (ret_library, ret_discard, ret_hand)
    }
    pub fn draw_to_size(
        library: &Vec<Card>,
        discard: &Vec<Card>,
        hand: &Vec<Card>,
        hand_size: &i64,
        chains: &i64,
    ) -> (Vec<Card>, Vec<Card>, Vec<Card>, i64) {
        let (ret_library, ret_discard, ret_hand) = Self::draw_to_given_size(
            &(hand_size - chains_amount(*chains)),
            library,
            discard,
            hand,
        );

        if (hand_size > &(ret_hand.len() as i64)) && (*chains > 0) {
            (ret_library, ret_discard, ret_hand, chains - 1)
        } else if (hand_size == &(ret_hand.len() as i64)) && (*chains == 0) {
            (ret_library, ret_discard, ret_hand, 0)
        } else if hand_size < &(ret_hand.len() as i64) {
            (ret_library, ret_discard, ret_hand, chains - 1)
        } else {
            panic!("ERR") // !err
        }
    }
    pub fn draw_to_given_size(
        size: &i64,
        library: &Vec<Card>,
        discard: &Vec<Card>,
        hand: &Vec<Card>,
    ) -> (Vec<Card>, Vec<Card>, Vec<Card>) {
        if size <= &(hand.len() as i64) {
            return (library.clone(), discard.clone(), hand.clone());
        }

        let (temp_library, temp_discard, temp_hand) = &Self::draw(library, discard, hand);
        Self::draw_to_given_size(size, temp_library, temp_discard, temp_hand)
    }

    pub fn remove_card_from_area(card: &Card, area: &Vec<Card>) -> Vec<Card> {
        let mut ret_area = area.clone();

        ret_area.remove(
            ret_area
                .iter()
                .position(|x| x == card)
                .expect("card not found"),
        );
        ret_area
    }
    pub fn add_card_to_area(card: &Card, area: &Vec<Card>) -> Vec<Card> {
        let mut ret_area = area.clone();

        ret_area.push(card.clone());
        ret_area
    }

    pub fn is_empty(&self) -> bool {
        return self.creature_board.is_none()
            && self.artifact_board.is_none()
            && self.hand.is_none()
            && self.archives.is_none()
            && self.library.is_none()
            && self.discard.is_none()
            && self.purged.is_none()
            && self.amber.is_none()
            && self.keys.is_none()
            && self.chains.is_none()
            && self.key_cost.is_none()
            && self.hand_size.is_none()
            && self.active_house.is_none();
    }
}

fn chains_amount(chains: i64) -> i64 {
    if chains >= 19 {
        return 4;
    } else if chains >= 13 {
        return 3;
    } else if chains >= 7 {
        return 2;
    } else if chains >= 1 {
        return 1;
    } else {
        return 0;
    }
}
