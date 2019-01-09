#[derive(Queryable)]
pub struct User {
    pub id: u64,
    pub lifetime_gross: i32,
    pub lifetime_net: i32,
    pub week_gross: i32,
    pub week_net: i32,
    pub tips: i32,
    pub tips_given: i32,
    pub anti_tips: i32,
    pub anti_tips_given: i32,
}

#[derive(Queryable)]
pub struct Tip {
    pub id: i32,
    pub user_from: u64,
    pub user_to: u64,
    pub time: u64,
    pub anti: bool,
}

#[derive(Queryable)]
pub struct Key {
    pub key: i64
}