use super::super::card_controller::Card;
use super::super::diff_controller::StateDiff;
use super::card_actions;
// // mod dew_faerie;
// // mod charette;

pub mod aaa_empty;

pub fn play(card: &Card, gs: &super::super::GameState) -> StateDiff {
    match card.hard_info.name {
        _ => match card.hard_info.card_type.as_str() {
            "Action" => card_actions::default_play_action_card(card, gs, None),
            // "Creature" => card_actions::default_play_creature_card(),
            // "Upgrade" => card_actions::default_play_upgrade_card(),
            // "Artifact" => card_actions::default_play_artifact_card(),
            _ => panic!("unrecognized or unsupported card type"),
        },
    }
}

pub fn discard(card: &Card, gs: &super::super::GameState) -> StateDiff {
    match card.hard_info.name {
        _ => card_actions::default_discard_card(card, gs, None),
    }
}

pub fn ready(card: &Card) -> Card {
    match card.hard_info.name {
        _ => card_actions::ready_card(card),
    }
}
