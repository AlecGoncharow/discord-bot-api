use diesel::{
    prelude::*,
    pg::PgConnection,
};
use iron::{status, IronResult, Request, Response};
use router::Router;
use regex::Regex;
use std::boxed::Box;

pub fn is_valid_key(conn: &PgConnection, provided_key: i64) -> bool {
    use crate::schema::keys::dsl::*;
    let results = keys.filter(key.eq(&provided_key))
        .load::<crate::models::Key>(conn)
        .expect("help");
    results.len() == 1
}

pub fn validate_key_test(req: &mut Request) -> IronResult<Response> {
    let conn = crate::establish_connection();
    let mut resp = Response::new();
    let query = req.url.query();
    let key_regex = Regex::new(r"key=([^&]+)").unwrap();

    match query {
        Some(q) =>{
            let caps = key_regex.captures(&q).unwrap();

            if caps.get(1).is_some() {
                let key: Result<i64, _> = String::from(caps.get(1).unwrap().as_str()).parse();
                match key {
                    Ok(k) => {
                        if is_valid_key(&conn, k ){
                            resp.status = Some(status::Ok);
                            resp.body = Some(Box::new("Valid key"));
                        } else {
                            resp.status = Some(status::Forbidden);
                            resp.body = Some(Box::new("Invalid key"))
                        }
                    }
                    Err(_) => {
                        resp.status = Some(status::Forbidden);
                        resp.body = Some(Box::new("Key must be an integer"));
                    }
                }

            } else {
                resp.body = Some(Box::new("No key query variable"));
            }
            resp.headers.set(iron::headers::ContentType::plaintext());
            Ok(resp)
        }
        None => {
            resp.body = Some(Box::new("Requires key query param"));
            resp.status = Some(status::Forbidden);
            resp.headers.set(iron::headers::ContentType::plaintext());
            Ok(resp)
        }
    }
}