diesel::table! {
    players (internal_id) {
        internal_id -> Integer,
        slap_id -> VarChar
    }
}

diesel::table! {
    matches (internal_id) {
        internal_id -> Integer,
        match_id -> VarChar
    }
}

diesel::table! {
    names (player_id, name) {
        player_id -> Integer,
        name -> VarChar
    }
}

diesel::table! {
    match_players (match_id, player_id) {
        match_id -> Integer,
        player_id -> Integer
    }
}

diesel::joinable!(names -> players (player_id));
//diesel::joinable!(matches -> match_players (internal_id));
//diesel::joinable!(players -> match_players (internal_id));

diesel::allow_tables_to_appear_in_same_query!(players, names);
diesel::allow_tables_to_appear_in_same_query!(matches, players, match_players);
