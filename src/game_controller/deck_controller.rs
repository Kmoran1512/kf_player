use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use serde_json;
use std::fs;

#[derive(Debug, Clone)]
pub struct Deck {
    pub owning_player_number: i64,

    pub name: String,
    pub houses: [String; 3],

    pub card_list: Vec<super::card_controller::Card>,
}
impl Deck {
    pub fn new_blank(
        owning_player_number: i64,
        deck_name: String,
        houses: Option<[&str; 3]>,
    ) -> Deck {
        let mut card_list: Vec<super::card_controller::Card> = Vec::new();

        let houses_unwrapped: [&str; 3] = houses.unwrap_or(["Brobnar", "Dis", "Shadows"]);

        for i in 0..36 {
            card_list.push(
                super::card_controller::card_list::aaa_empty::new_empty_action(
                    owning_player_number,
                    houses_unwrapped[i / 12].to_string(),
                    None,
                    None,
                    None,
                    None,
                ),
            )
        }

        Deck {
            owning_player_number: owning_player_number,
            name: deck_name,
            houses: [
                houses_unwrapped[0].to_string(),
                houses_unwrapped[1].to_string(),
                houses_unwrapped[2].to_string(),
            ],
            card_list: card_list,
        }
    }

    pub fn new_from_string(owning_player_number: i64, deck_id: &str) -> Deck {
        // TODO-KM: Make this nicer
        let deck_location_string = format!(
            "/media/kmoran/My Passport/Kyle/Code/AI stuffs/smiling_ruth/Decks/{}.json",
            clean_deck_string(deck_id)
        );

        let contents = fs::read_to_string(deck_location_string)
            .expect("Something went wrong reading the file"); // TODO-KM: API request
        let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let deck_name: String = parsed["data"]["name"].as_str().unwrap().to_string();
        let mut houses: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
        let houses_api = parsed["data"]["_links"]["houses"].as_array().unwrap();
        for i in 0..3 {
            houses[i] = houses_api[i].as_str().unwrap().to_string();
        }

        let cards_id_list = parsed["data"]["_links"]["cards"].as_array().unwrap();
        let cards_api_list = parsed["_linked"]["cards"].as_array().unwrap();

        let mut card_list: Vec<super::card_controller::Card> = Vec::new();

        for i in 0..cards_id_list.len() {
            for card_api in cards_api_list {
                if card_api["id"] == cards_id_list[i] {
                    card_list.push(super::card_controller::Card::new(
                        owning_player_number,
                        card_api,
                    ));
                }
            }
        }

        let deck: Deck = Deck {
            owning_player_number: owning_player_number,
            name: deck_name,
            houses: houses,
            card_list: card_list,
        };

        deck
    }

    pub fn library_from_list(&self) -> Vec<super::card_controller::Card> {
        let mut library = self.card_list.clone();
        library.shuffle(&mut thread_rng());

        library
    }
}

fn clean_deck_string(deck_link: &str) -> &str {
    let re = Regex::new(
        r"[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}",
    )
    .unwrap();

    re.find(deck_link).unwrap().as_str()
}
