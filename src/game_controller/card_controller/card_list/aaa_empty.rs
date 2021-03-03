use uuid::Uuid;

pub fn new_empty_action(
    owning_player_number: i64,
    house: String,
    alpha: Option<bool>,
    omega: Option<bool>,
    amber: Option<i64>,
    enhancements: Option<[u8; 4]>,
) -> super::super::super::card_controller::Card {
    let keywords = super::super::Keywords {
        alpha: alpha.unwrap_or(false),
        assault_x: 0,
        deploy: false,
        elusive: false,
        hazardous_x: 0,
        omega: omega.unwrap_or(false),
        poison: false,
        skirmish: false,
        taunt: false,
    };
    let hard_info = super::super::HardInfo {
        owning_player_number: owning_player_number,
        card_id: Uuid::new_v4(),
        name: "Empty Action".to_string(),
        house: house,
        card_type: "Action".to_string(),
        traits: vec![],
        amber: amber.unwrap_or(1),
        enhancements: enhancements.unwrap_or([0, 0, 0, 0]),
        card_number: 0,
        expansion: "None",
        power: 0,
        armor: 0,
        keywords: keywords,
    };

    super::super::super::card_controller::Card {
        hard_info: hard_info.clone(),
        name: "Empty Action".to_string(),

        house: hard_info.house.clone(),
        card_type: "Action".to_string(),

        controller_number: hard_info.owning_player_number,

        traits: hard_info.traits.clone(),

        is_ready: false,

        current_power: 0,
        damage: 0,

        max_armor: 0,
        current_armor: 0,

        amber_stored: 0,

        keywords: keywords,
    }
}
