use crate::models::{Tip, User};
use diesel::{pg::PgConnection, prelude::*};
use iron::{status, IronResult, Request, Response};
use regex::Regex;
use router::Router;
use std::boxed::Box;
use std::time::{SystemTime, UNIX_EPOCH};

pub enum KeyState {
    Ok,
    Invalid,
    NotI64,
    MissingVar,
}

pub fn is_valid_key(conn: &PgConnection, provided_key: i64) -> bool {
    use crate::schema::keys::dsl::*;
    let results = keys
        .find(provided_key)
        .load::<crate::models::Key>(conn)
        .expect("help");
    results.len() == 1
}

pub fn get_user(conn: &PgConnection, provided_id: i64) -> Vec<User> {
    use crate::schema::users::dsl::*;
    users
        .find(provided_id)
        .load::<User>(conn)
        .expect("Error loading users")
}

pub fn get_user_view(req: &mut Request) -> IronResult<Response> {
    let conn = crate::establish_connection();
    let mut resp = Response::new();
    match get_id(req, "user") {
        Ok(val) => {
            let user = get_user(&conn, val);
            if user.len() == 1 {
                let json_value = serde_json::to_value(&user[0]).unwrap();
                resp.body = Some(Box::new(json_value.to_string()));
                resp.status = Some(status::Ok);
                resp.headers.set(iron::headers::ContentType::json());

                Ok(resp)
            } else {
                resp.status = Some(status::NotFound);
                Ok(resp)
            }
        }
        Err(_) => {
            resp.status = Some(status::BadRequest);
            Ok(resp)
        }
    }
}

pub fn get_id(req: &mut Request, key: &str) -> Result<i64, <i64 as std::str::FromStr>::Err> {
    let params = req.extensions.get::<Router>().unwrap();
    params.find(key).unwrap().to_string().parse()
}

pub fn update_user(conn: &PgConnection, user: &User) {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .filter(id.eq(user.id))
        .set((
            lifetime_net.eq(user.lifetime_net),
            lifetime_gross.eq(user.lifetime_gross),
            week_net.eq(user.week_net),
            week_gross.eq(user.week_gross),
            tips.eq(user.tips),
            tips_given.eq(user.tips_given),
            anti_tips.eq(user.anti_tips),
            anti_tips_given.eq(user.anti_tips_given),
        ))
        .execute(conn);
}

pub fn create_user(conn: &PgConnection, provided_id: i64) -> User {
    use crate::schema::users::dsl::*;
    let mut user = User::default();
    user.id = provided_id;
    diesel::insert_into(users).values(&user).execute(conn);
    user
}

pub fn create_user_view(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.headers.set(iron::headers::ContentType::plaintext());
    match validate_key(req) {
        KeyState::Ok => match get_id(req, "user") {
            Ok(val) => {
                let conn = crate::establish_connection();
                create_user(&conn, val);
                resp.body = Some(Box::new("User Created!"));
                resp.status = Some(status::Ok);
                Ok(resp)
            }
            Err(_) => {
                resp.body = Some(Box::new("Bad Request"));
                resp.status = Some(status::BadRequest);
                Ok(resp)
            }
        },
        _ => {
            resp.body = Some(Box::new("Forbidden"));
            resp.status = Some(status::Forbidden);
            Ok(resp)
        }
    }
}

pub fn set_tips(conn: &PgConnection, provided_id: i64, new_tips: i32) {
    use crate::schema::users::dsl::*;
    if provided_id == -1 {
        diesel::update(users).set(tips.eq(new_tips)).execute(conn);
    } else {
        diesel::update(users)
            .filter(id.eq(provided_id))
            .set(tips.eq(new_tips))
            .execute(conn);
    }
}

pub fn set_anti_tips(conn: &PgConnection, provided_id: i64, new_tips: i32) {
    use crate::schema::users::dsl::*;
    if provided_id == -1 {
        diesel::update(users)
            .set(anti_tips.eq(new_tips))
            .execute(conn);
    } else {
        diesel::update(users)
            .filter(id.eq(provided_id))
            .set(anti_tips.eq(new_tips))
            .execute(conn);
    }
}

pub fn set_tips_view(req: &mut Request, is_anti: bool) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.headers.set(iron::headers::ContentType::plaintext());
    match validate_key(req) {
        KeyState::Ok => match get_id(req, "user") {
            Ok(user) => match get_id(req, "val") {
                Ok(val) => {
                    let conn = crate::establish_connection();
                    if is_anti {
                        set_anti_tips(&conn, user, val as i32);
                    } else {
                        set_tips(&conn, user, val as i32);
                    }
                    resp.body = Some(Box::new("Tips set"));
                    resp.status = Some(status::Ok);
                    Ok(resp)
                }
                Err(_) => {
                    resp.body = Some(Box::new("Bad Request"));
                    resp.status = Some(status::BadRequest);
                    Ok(resp)
                }
            },
            Err(_) => {
                resp.body = Some(Box::new("Bad Request"));
                resp.status = Some(status::BadRequest);
                Ok(resp)
            }
        },
        _ => {
            resp.body = Some(Box::new("Forbidden"));
            resp.status = Some(status::Forbidden);
            Ok(resp)
        }
    }
}

pub enum TipState {
    Ok(Tip),
    SameId,
    NoTips,
}

pub fn transact_tip_view(req: &mut Request, is_anti: bool) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.headers.set(iron::headers::ContentType::plaintext());
    match validate_key(req) {
        KeyState::Ok => match get_id(req, "from") {
            Ok(from) => match get_id(req, "to") {
                Ok(to) => {
                    let conn = crate::establish_connection();
                    let mut from_user = get_user(&conn, from);
                    if from_user.len() != 1 {
                        resp.body = Some(Box::new("Not Found, From User"));
                        resp.status = Some(status::NotFound);
                        return Ok(resp);
                    }
                    let mut to_user = get_user(&conn, to);
                    if to_user.len() != 1 {
                        resp.body = Some(Box::new("Not Found, To User"));
                        resp.status = Some(status::NotFound);
                        return Ok(resp);
                    }

                    if is_anti {
                        match transact_anti_tip(&conn, &mut from_user[0], &mut to_user[0]) {
                            TipState::Ok(tip) => {
                                let json_value = serde_json::to_value(&tip).unwrap();
                                resp.body = Some(Box::new(json_value.to_string()));
                                resp.status = Some(status::Ok);
                                resp.headers.set(iron::headers::ContentType::json());
                            }
                            TipState::NoTips => {
                                resp.body = Some(Box::new("No Tips"));
                                resp.status = Some(status::NotAcceptable);
                            }
                            TipState::SameId => {
                                resp.body = Some(Box::new("Same ID"));
                                resp.status = Some(status::NotAcceptable);
                            }
                        }
                    } else {
                        match transact_tip(&conn, &mut from_user[0], &mut to_user[0]) {
                            TipState::Ok(tip) => {
                                let json_value = serde_json::to_value(&tip).unwrap();
                                resp.body = Some(Box::new(json_value.to_string()));
                                resp.status = Some(status::Ok);
                                resp.headers.set(iron::headers::ContentType::json());
                            }
                            TipState::NoTips => {
                                resp.body = Some(Box::new("No Tips"));
                                resp.status = Some(status::NotAcceptable);
                            }
                            TipState::SameId => {
                                resp.body = Some(Box::new("Same ID"));
                                resp.status = Some(status::NotAcceptable);
                            }
                        }
                    }

                    Ok(resp)
                }
                Err(_) => {
                    resp.body = Some(Box::new("Bad Request"));
                    resp.status = Some(status::BadRequest);
                    Ok(resp)
                }
            },
            Err(_) => {
                resp.body = Some(Box::new("Bad Request"));
                resp.status = Some(status::BadRequest);
                Ok(resp)
            }
        },
        _ => {
            resp.body = Some(Box::new("Forbidden"));
            resp.status = Some(status::Forbidden);
            Ok(resp)
        }
    }
}

pub fn create_tip(conn: &PgConnection, from: &mut User, to: &mut User, is_anti: bool) -> Tip {
    use crate::schema::tips::dsl::*;
    let curr_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let res = tips
        .filter(id.ne(0))
        .load::<Tip>(conn)
        .expect("Error reading tips");
    let tip_id = if res.len() == 0 {
        1
    } else {
        res.iter().max_by(|x, y| x.id.cmp(&y.id)).unwrap().id + 1
    };
    let tip = Tip {
        id: tip_id,
        user_from: from.id,
        user_to: to.id,
        time: curr_time.as_secs() as i64,
        anti: is_anti,
    };

    diesel::insert_into(tips).values(&tip).execute(conn);
    tip
}

pub fn transact_tip(conn: &PgConnection, from: &mut User, to: &mut User) -> TipState {
    if from.id == to.id {
        return TipState::SameId;
    }
    if from.tips < 1 {
        return TipState::NoTips;
    }

    from.tips -= 1;
    from.tips_given += 1;
    update_user(conn, from);

    to.lifetime_gross += 1;
    to.lifetime_net += 1;
    to.week_gross += 1;
    to.week_net += 1;
    update_user(conn, to);

    TipState::Ok(create_tip(conn, from, to, false))
}

pub fn transact_anti_tip(conn: &PgConnection, from: &mut User, to: &mut User) -> TipState {
    if from.id == to.id {
        return TipState::SameId;
    }
    if from.anti_tips < 1 {
        return TipState::NoTips;
    }
    if to.lifetime_net == 0 {
        return TipState::NoTips;
    }
    if to.week_net == 0 {
        return TipState::NoTips;
    }
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    from.anti_tips -= 1;
    from.anti_tips_given += 1;
    update_user(conn, from);

    to.lifetime_net -= 1;
    to.week_net -= 1;
    update_user(conn, to);

    TipState::Ok(create_tip(conn, from, to, true))
}

pub fn validate_key(req: &mut Request) -> KeyState {
    let conn = crate::establish_connection();
    let query = req.url.query();
    let key_regex = Regex::new(r"key=([^&]+)").unwrap();

    match query {
        Some(q) => {
            let caps = key_regex.captures(&q);

            if caps.is_some() {
                let unwrapped = caps.unwrap();
                let key: Result<i64, _> = String::from(unwrapped.get(1).unwrap().as_str()).parse();
                match key {
                    Ok(k) => {
                        if is_valid_key(&conn, k) {
                            KeyState::Ok
                        } else {
                            KeyState::Invalid
                        }
                    }
                    Err(_) => KeyState::NotI64,
                }
            } else {
                KeyState::MissingVar
            }
        }
        None => KeyState::MissingVar,
    }
}

pub fn validate_key_test(req: &mut Request) -> IronResult<Response> {
    let state = validate_key(req);
    let mut resp = Response::new();
    resp.headers.set(iron::headers::ContentType::plaintext());

    match state {
        KeyState::Ok => {
            resp.status = Some(status::Ok);
            resp.body = Some(Box::new("Valid key"));
        }
        KeyState::Invalid => {
            resp.status = Some(status::Forbidden);
            resp.body = Some(Box::new("Invalid key"))
        }
        KeyState::NotI64 => {
            resp.status = Some(status::Forbidden);
            resp.body = Some(Box::new("Key must be an integer"));
        }
        KeyState::MissingVar => {
            resp.body = Some(Box::new("No key query variable"));
        }
    }
    Ok(resp)
}
