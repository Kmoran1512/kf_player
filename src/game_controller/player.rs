use super::move_controller;

use super::card_controller::Card;
use super::deck_controller::Deck;

#[derive(Debug, Clone)]
pub struct Player {
    pub brain: super::move_controller::Minds,
    pub name: String,
    pub deck: Deck,
    pub player_number: i64,

    pub creature_board: Vec<Card>,
    pub artifact_board: Vec<Card>,

    pub hand: Vec<Card>,
    pub archives: Vec<Card>,
    pub library: Vec<Card>,
    pub discard: Vec<Card>,
    pub purged: Vec<Card>,

    pub amber: i64,
    pub keys: Vec<&'static str>,
    pub chains: i64,

    pub key_cost: i64,
    pub hand_size: i64,

    pub active_house: Option<String>,
}
impl Player {
    // pub fn new(
    //     name: &str,
    //     deck: Deck,
    //     player_number: u8,
    //     brain: move_controller::Minds,
    // ) -> Player {
    //     Player {
    //         brain: brain,
    //         name: name.to_string(),
    //         deck: deck,
    //         player_number: player_number,
    //         creature_board: Vec::new(),
    //         artifact_board: Vec::new(),
    //         hand: Vec::new(),
    //         archives: Vec::new(),
    //         library: Vec::new(),
    //         discard: Vec::new(),
    //         purged: Vec::new(),
    //         amber: 0,
    //         keys: vec!["red", "blue", "yellow"],
    //         chains: 0,
    //         key_cost: 6,
    //         hand_size: 6,
    //     }
    // }

    pub fn empty_new(player_number: i64) -> Player {
        Player {
            brain: move_controller::Minds::Lollop,
            name: "lollop player".to_string(),
            deck: Deck::new_blank(player_number, "Empty Deck".to_string(), None),
            player_number: 0,
            creature_board: vec![],
            artifact_board: vec![],
            hand: vec![],
            archives: vec![],
            library: vec![],
            discard: vec![],
            purged: vec![],
            amber: 0,
            keys: vec!["red", "blue", "yellow"],
            chains: 0,
            key_cost: 6,
            hand_size: 6,
            active_house: None,
        }
    }

    pub fn set_up(&mut self, first_player: Option<i64>) {
        self.library = self.deck.library_from_list();
        self.hand_size += first_player.unwrap_or(0);
        self.player_number = first_player.unwrap_or(0);

        let mut pd: super::diff_controller::PlayerDiff = super::diff_controller::PlayerDiff::new();
        let (temp_library, temp_discard, temp_hand, temp_chains) =
            super::diff_controller::PlayerDiff::draw_to_size(
                &self.library,
                &self.discard,
                &self.hand,
                &self.hand_size,
                &self.chains,
            );

        pd.library = Some(temp_library);
        pd.discard = Some(temp_discard);
        pd.hand = Some(temp_hand);
        if self.chains != temp_chains {
            pd.chains = Some(temp_chains)
        }

        self.add(&pd);
    }

    pub fn add(&mut self, pd: &super::diff_controller::PlayerDiff) {
        if !pd.creature_board.is_none() {
            self.creature_board = pd.creature_board.clone().unwrap()
        }
        if !pd.artifact_board.is_none() {
            self.artifact_board = pd.artifact_board.clone().unwrap();
        }
        if !pd.hand.is_none() {
            self.hand = pd.hand.clone().unwrap();
        }
        if !pd.archives.is_none() {
            self.archives = pd.archives.clone().unwrap();
        }
        if !pd.library.is_none() {
            self.library = pd.library.clone().unwrap();
        }
        if !pd.discard.is_none() {
            self.discard = pd.discard.clone().unwrap();
        }
        if !pd.purged.is_none() {
            self.purged = pd.purged.clone().unwrap();
        }

        if !pd.keys.is_none() {
            self.keys = pd.keys.clone().unwrap();
        }

        self.amber = pd.amber.unwrap_or(self.amber);
        self.chains = pd.chains.unwrap_or(self.chains);
        self.key_cost = pd.key_cost.unwrap_or(self.key_cost);
        self.hand_size = pd.hand_size.unwrap_or(self.hand_size);

        let tempie = pd.active_house.clone();
        let temp2 = tempie.unwrap_or(self.active_house.clone());
        self.active_house = temp2; // This was causing problems, so the above was done
    }
}
