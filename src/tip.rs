use diesel::{
    prelude::*,
    pg::PgConnection,
};
use iron::{status, IronResult, Request, Response};
use regex::Regex;
use std::boxed::Box;
use router::Router;
use crate::models::{User, Tip};

pub enum KeyState {
    Ok,
    Invalid,
    NotI64,
    MissingVar,
}

pub fn is_valid_key(conn: &PgConnection, provided_key: i64) -> bool {
    use crate::schema::keys::dsl::*;
    let results = keys.find(provided_key)
        .load::<crate::models::Key>(conn)
        .expect("help");
    results.len() == 1
}

pub fn get_user(conn: &PgConnection, provided_id: i64) -> Vec<User> {
    use crate::schema::users::dsl::*;
    users.find(provided_id)
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
        Err(_) =>{
            resp.status = Some(status::BadRequest);
            Ok(resp)
        }
    }
}

pub fn get_id(req: &mut Request, key: &str) -> Result<i64, <i64 as std::str::FromStr>::Err> {
    let params = req.extensions.get::<Router>().unwrap();
    params.find(key).unwrap().to_string().parse()
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
        KeyState::Ok => {
            match get_id(req, "user") {
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
            }
        }
        _ => {
            resp.body = Some(Box::new("Forbidden"));
            resp.status = Some(status::Forbidden);
            Ok(resp)
        }
    }
}

pub fn validate_key(req: &mut Request) -> KeyState {
    let conn = crate::establish_connection();
    let query = req.url.query();
    let key_regex = Regex::new(r"key=([^&]+)").unwrap();

    match query {
        Some(q) =>{
            let caps = key_regex.captures(&q);

            if caps.is_some() {
                let unwrapped = caps.unwrap();
                let key: Result<i64, _> = String::from(unwrapped.get(1).unwrap().as_str()).parse();
                match key {
                    Ok(k) => {
                        if is_valid_key(&conn, k ){
                            KeyState::Ok
                        } else {
                            KeyState::Invalid
                        }
                    }
                    Err(_) => {
                        KeyState::NotI64
                    }
                }
            } else {
                KeyState::MissingVar
            }
        }
        None => {
            KeyState::MissingVar
        }
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
