// @generated automatically by Diesel CLI.

diesel::table! {
    bets (id) {
        id -> Integer,
        fk_games -> Integer,
        fk_teams -> Integer,
        fk_users -> Integer,
        nb_nut -> Integer,
    }
}

diesel::table! {
    games (id) {
        id -> Integer,
        fk_tournaments -> Integer,
        fk_team1 -> Integer,
        fk_team2 -> Integer,
        score1 -> Integer,
        score2 -> Integer,
        phase -> Integer,
        place -> Integer,
        status -> Integer,
        has_gained_nut -> Bool,
    }
}

diesel::table! {
    nuts (id) {
        id -> Integer,
        fk_users -> Integer,
        fk_tournaments -> Integer,
        stock -> Integer,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Integer,
        fk_users -> Integer,
        fk_tournaments -> Integer,
    }
}

diesel::table! {
    teams (id) {
        id -> Integer,
        fk_tournaments -> Integer,
        #[max_length = 255]
        name -> Varchar,
        group -> Integer,
    }
}

diesel::table! {
    tokens (token) {
        #[max_length = 255]
        token -> Varchar,
        fk_users -> Integer,
        created_at -> Datetime,
        expiration_date -> Datetime,
    }
}

diesel::table! {
    tournaments (id) {
        id -> Integer,
        fk_users -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        date -> Datetime,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        phase -> Integer,
        size_group -> Nullable<Integer>,
        #[max_length = 16]
        code -> Varchar,
        is_qualif -> Bool,
        is_elim -> Bool,
        is_closed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(bets -> games (fk_games));
diesel::joinable!(bets -> teams (fk_teams));
diesel::joinable!(bets -> users (fk_users));
diesel::joinable!(games -> tournaments (fk_tournaments));
diesel::joinable!(nuts -> tournaments (fk_tournaments));
diesel::joinable!(nuts -> users (fk_users));
diesel::joinable!(subscriptions -> tournaments (fk_tournaments));
diesel::joinable!(subscriptions -> users (fk_users));
diesel::joinable!(teams -> tournaments (fk_tournaments));
diesel::joinable!(tokens -> users (fk_users));
diesel::joinable!(tournaments -> users (fk_users));

diesel::allow_tables_to_appear_in_same_query!(
    bets,
    games,
    nuts,
    subscriptions,
    teams,
    tokens,
    tournaments,
    users,
);
