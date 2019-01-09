use schema::{users, tips};
pub const WEEKLY_TIPS: i32 = 7;
pub const WEEKLY_ANTI_TIPS: i32 = 1;

#[derive(Queryable, Insertable, Debug, Serialize)]
#[table_name="users"]
pub struct User {
    pub id: i64,
    pub lifetime_gross: i32,
    pub lifetime_net: i32,
    pub week_gross: i32,
    pub week_net: i32,
    pub tips: i32,
    pub tips_given: i32,
    pub anti_tips: i32,
    pub anti_tips_given: i32,
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            lifetime_gross: 0,
            lifetime_net: 0,
            week_gross: 0,
            week_net: 0,
            tips: WEEKLY_TIPS,
            tips_given: 0,
            anti_tips: WEEKLY_ANTI_TIPS,
            anti_tips_given: 0,
        }
    }
}

#[derive(Queryable, Insertable, Debug, Serialize)]
#[table_name="tips"]
pub struct Tip {
    pub id: i32,
    pub user_from: i64,
    pub user_to: i64,
    pub time: i64,
    pub anti: bool,
}

#[derive(Queryable)]
pub struct Key {
    pub key: i64
}