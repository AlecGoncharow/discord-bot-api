table! {
    keys (key) {
        key -> Int8,
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
        lifetime_gross -> Nullable<Int4>,
        lifetime_net -> Nullable<Int4>,
        week_gross -> Nullable<Int4>,
        week_net -> Nullable<Int4>,
        tips -> Nullable<Int4>,
        tips_given -> Nullable<Int4>,
        anti_tips -> Nullable<Int4>,
        anti_tips_given -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    keys,
    tips,
    users,
);
