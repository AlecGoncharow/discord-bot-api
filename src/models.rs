#[derive(Queryable)]
pub struct User {
    pub id: u64,
    pub lifetime_gross: i32,
    pub lifetime_net: i32,
    pub week_gross: i8,
    pub week_net: i8,
    pub tips: u8,
    pub tips_given: u32,
    pub anti_tips: u8,
    pub anti_tips_given: u32,
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