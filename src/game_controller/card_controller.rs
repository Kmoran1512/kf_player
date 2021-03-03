pub mod card_actions;
pub mod card_list;

use regex::Regex;
use serde_json;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Card {
    pub hard_info: HardInfo,
    pub name: String,

    pub house: String,
    pub card_type: String,

    pub controller_number: i64,

    pub traits: Vec<String>,

    pub is_ready: bool,

    pub current_power: i64,
    pub damage: i64,

    pub max_armor: i64,
    pub current_armor: i64,

    pub amber_stored: i64,

    pub keywords: Keywords,
}
impl Card {
    pub fn new(owning_player_number: i64, card_api: &serde_json::Value) -> Card {
        let hard_info = HardInfo {
            card_id: Uuid::new_v4(),
            owning_player_number: owning_player_number,
            name: card_api["card_title"].as_str().unwrap().to_string(),
            house: card_api["house"].as_str().unwrap().to_string(),
            card_type: card_api["card_type"].as_str().unwrap().to_string(),
            traits: solve_traits(card_api["card_type"].as_str().unwrap()),
            amber: card_api["amber"].as_i64().unwrap(),
            enhancements: solve_enhancements(card_api["is_enhanced"].as_bool().unwrap()),
            card_number: card_api["card_number"]
                .as_str()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            expansion: solve_expansion(card_api["expansion"].as_u64().unwrap()),
            power: card_api["power"]
                .as_str()
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap(),
            armor: card_api["armor"]
                .as_str()
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap(),
            keywords: set_keywords(card_api["card_text"].as_str().unwrap()),
        };

        Card {
            hard_info: hard_info.clone(),
            name: hard_info.name.clone(),
            house: hard_info.house.clone(),
            controller_number: hard_info.owning_player_number,
            card_type: hard_info.card_type.clone(),
            traits: hard_info.traits.clone(),
            is_ready: false,
            current_power: hard_info.power,
            damage: 0,
            max_armor: hard_info.armor,
            current_armor: hard_info.armor,
            amber_stored: 0,
            keywords: hard_info.keywords,
        }
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.hard_info.card_id == other.hard_info.card_id
    }
}
impl Eq for Card {}

#[derive(Debug, Clone)]
pub struct HardInfo {
    pub owning_player_number: i64,
    pub card_id: Uuid,

    pub name: String,
    pub house: String,
    pub card_type: String,

    pub traits: Vec<String>,

    pub amber: i64,
    pub enhancements: [u8; 4],

    pub card_number: i64,
    pub expansion: &'static str,

    pub power: i64,
    pub armor: i64,

    pub keywords: Keywords,
}

#[derive(Debug, Clone, Copy)]
pub struct Keywords {
    pub alpha: bool,
    pub assault_x: u64,
    pub deploy: bool,
    pub elusive: bool,
    pub hazardous_x: u64,
    pub omega: bool,
    pub poison: bool,
    pub skirmish: bool,
    pub taunt: bool,
}

fn solve_expansion(expansion_code: u64) -> &'static str {
    match expansion_code {
        341 => return "Call of the Archons",
        435 => return "Age of Ascension",
        452 => return "Worlds Collide",
        453 => return "Anomaly...",
        479 => return "Mass Mutation",
        _ => panic!("Not a valid set code"),
    }
}

fn solve_traits(traits_str: &str) -> Vec<String> {
    traits_str
        .split("•")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}

fn solve_enhancements(_placeholder: bool) -> [u8; 4] {
    // TODO-KM: I have no clue how to do this...
    [0, 0, 0, 0]
}

fn set_keywords(card_text: &str) -> Keywords {
    let assault_regex = Regex::new(r"Assault [0-9]\.").unwrap();
    let hazardous_regex = Regex::new(r"Hazardous [0-9]\.").unwrap();

    let assualt_check = assault_regex.find(card_text);
    let hazard_check = hazardous_regex.find(card_text);

    let assault_x = if assualt_check.is_none() {
        0
    } else {
        get_number_from_keyword(assualt_check.unwrap().as_str())
    };

    let hazard_x = if hazard_check.is_none() {
        0
    } else {
        get_number_from_keyword(hazard_check.unwrap().as_str())
    };

    Keywords {
        alpha: card_text.contains("Alpha."),
        assault_x: assault_x,
        deploy: card_text.contains("Deploy."),
        elusive: card_text.contains("Elusive."),
        hazardous_x: hazard_x,
        omega: card_text.contains("Omega."),
        poison: card_text.contains("Poison."),
        skirmish: card_text.contains("Skirmish."),
        taunt: card_text.contains("Taunt."),
    }
}

fn get_number_from_keyword(keyword_text: &str) -> u64 {
    let number_regex = Regex::new(r"[0-9]+").unwrap();

    number_regex
        .find(keyword_text)
        .unwrap()
        .as_str()
        .parse::<u64>()
        .unwrap()
}
