use super::super::card_controller::Card;
use super::super::diff_controller::{PlayerDiff, StateDiff};

pub fn default_play_action_card(
    card: &Card,
    gs: &super::super::GameState,
    additional_fn: Option<&dyn Fn(&super::Card, &super::super::GameState) -> StateDiff>,
) -> StateDiff {
    let mut temp_gs = gs.clone();

    if card.hard_info.amber > 0 {
        temp_gs.add(&super::super::game_actions::gain_amber(
            card.hard_info.owning_player_number,
            gs,
            card.hard_info.amber, // TODO: enhancements
        ));
    }

    if !(additional_fn.is_none()) {
        temp_gs.add(&additional_fn.unwrap()(card, gs));
    };

    let mut ret_sd = StateDiff::subtract(gs, &temp_gs);
    let mut ret_pd = ret_sd
        .get_player_by_number(card.hard_info.owning_player_number)
        .clone();

    ret_pd.hand = Some(PlayerDiff::remove_card_from_area(
        card,
        &temp_gs
            .get_player_by_number(card.hard_info.owning_player_number)
            .hand,
    ));
    ret_pd.discard = Some(PlayerDiff::add_card_to_area(
        card,
        &temp_gs
            .get_player_by_number(card.hard_info.owning_player_number)
            .discard,
    ));

    ret_sd.insert_playerdiff_by_number(ret_pd, card.hard_info.owning_player_number);
    ret_sd
}

pub fn default_discard_card(
    card: &Card,
    gs: &super::super::GameState,
    additional_fn: Option<&dyn Fn(&super::Card, &super::super::GameState) -> StateDiff>,
) -> StateDiff {
    let mut temp_gs = gs.clone();

    if !(additional_fn.is_none()) {
        temp_gs.add(&additional_fn.unwrap()(card, gs));
    };

    let mut ret_sd = StateDiff::set_equal(&temp_gs);
    let mut ret_pd =
        PlayerDiff::set_equal(&temp_gs.get_player_by_number(card.hard_info.owning_player_number));

    ret_pd.hand = Some(PlayerDiff::remove_card_from_area(
        card,
        &ret_pd.hand.unwrap(),
    ));
    ret_pd.discard = Some(PlayerDiff::add_card_to_area(card, &ret_pd.discard.unwrap()));

    ret_sd.insert_playerdiff_by_number(ret_pd, card.hard_info.owning_player_number);
    ret_sd
}

pub fn ready_card(card: &Card) -> Card {
    // TODO: Maybe check for freeze effects?
    let mut ret_card = card.clone();
    ret_card.is_ready = true;
    ret_card
}
