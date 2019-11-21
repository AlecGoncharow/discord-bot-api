table! {
    keys (key) {
        key -> Int8,
    }
}

table! {
    times (id) {
        id -> Int4,
        last_reset_time -> Int8,
    }
}

table! {
    tips (id) {
        id -> Int4,
        user_from -> Int8,
        user_to -> Int8,
        time -> Int8,
        anti -> Bool,
    }
}

table! {
    users (id) {
        id -> Int8,
        lifetime_gross -> Int4,
        lifetime_net -> Int4,
        week_gross -> Int4,
        week_net -> Int4,
        tips -> Int4,
        tips_given -> Int4,
        anti_tips -> Int4,
        anti_tips_given -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    keys,
    times,
    tips,
    users,
);
