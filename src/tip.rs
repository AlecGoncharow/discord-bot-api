use diesel::{
    prelude::*,
    pg::PgConnection,
};
use iron::{status, IronResult, Request, Response};
use regex::Regex;
use std::boxed::Box;

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
