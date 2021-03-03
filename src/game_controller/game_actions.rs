use super::diff_controller::{PlayerDiff, StateDiff};

pub fn steal_amber(
    from_player_number: i64,
    to_player_number: i64,
    gs: &super::GameState,
    amount: i64,
) -> StateDiff {
    let lost_state = lose_amber(from_player_number, gs, amount);
    let mut gained_state = gain_amber(
        to_player_number,
        gs,
        gs.get_player_by_number(from_player_number).amber
            - lost_state
                .get_player_by_number(from_player_number)
                .amber
                .unwrap(),
    );
    gained_state.insert_playerdiff_by_number(
        lost_state.get_player_by_number(from_player_number).clone(),
        from_player_number as i64,
    );

    gained_state
}

pub fn gain_amber(player_number: i64, gs: &super::GameState, amount: i64) -> StateDiff {
    let mut pd = PlayerDiff::new();
    pd.amber = Some(gs.get_player_by_number(player_number).amber + amount);

    let mut gd = StateDiff::new();
    gd.insert_playerdiff_by_number(pd, player_number);
    gd
}

pub fn lose_amber(player_number: i64, gs: &super::GameState, amount: i64) -> StateDiff {
    let mut pd = PlayerDiff::new();

    let amount_left: i64 = if gs.get_player_by_number(player_number).amber > amount {
        gs.get_player_by_number(player_number).amber - amount
    } else {
        0
    };

    pd.amber = Some(amount_left);

    let mut gd = StateDiff::new();
    gd.insert_playerdiff_by_number(pd, player_number);
    gd
}
