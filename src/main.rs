#![allow(proc_macro_derive_resolution_fallback)]
extern crate base64;
extern crate iron;
extern crate mount;
extern crate regex;
extern crate router;
extern crate staticfile;
//#[macro_use]
extern crate artifact_lib;
extern crate artifact_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
mod artifact;
pub mod models;
pub mod schema;
pub mod tip;

use iron::{status, Iron, IronResult, Request, Response};
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::env;
use std::path::Path;

use diesel::{pg::PgConnection, prelude::*};
// Serves a string to the user.  Try accessing "/".
fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Hello world!"));
    Ok(resp)
}

// Serves a customized string to the user.  Try accessing "/world".
fn hello_name(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let name = params.find("name").unwrap();
    let resp = Response::with((status::Ok, format!("Hello, {}!", name)));
    Ok(resp)
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn establish_connection() -> PgConnection {
    let data_base_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    PgConnection::establish(&data_base_url)
        .expect(&format!("Error connecting to {}", data_base_url))
}

/// Configure and run our server.
fn main() {
    // Set up our URL router.
    let mut router: Router = Router::new();
    router.get("/", hello, "index");
    router.get("/:name", hello_name, "name");
    router.get(
        "/artifact/decks/decode/:adc",
        artifact::decode_and_return_json,
        "adc_decode",
    );
    router.get("/tips/key_test/", tip::validate_key_test, "tip_id");
    router.get("/tips/user/:user/", tip::get_user_view, "tip_user");

    router.get(
        "/tips/user/create/:user",
        tip::create_user_view,
        "tip_user_create",
    );

    router.get(
        "/tips/user/:user/set_tips/:val",
        move |request: &mut Request| tip::set_tips_view(request, false),
        "set_tips",
    );

    router.get(
        "/tips/user/:user/set_anti_tips/:val",
        move |request: &mut Request| tip::set_tips_view(request, true),
        "set_anti_tips",
    );

    router.get("/tips/data",
                tip::get_data_view,
                "get_data");

    router.get(
        "/tips/tip/:from/:to",
        move |request: &mut Request| tip::transact_tip_view(request, false),
        "tip_from_to",
    );
    router.get(
        "/tips/anti_tip/:from/:to",
        move |request: &mut Request| tip::transact_tip_view(request, true),
        "anti_tip_from_to",
    );

    match std::env::var("HEROKU") {
        Ok(val) => {
            if val == "true" {
                match std::fs::create_dir("/app/.cache/") {
                    Ok(_) => println!("Cache created"),
                    Err(e) => panic!(format!("error creating cache: {}", e)),
                }
            }
        }
        Err(_) => {}
    }

    let connection = establish_connection();
    tip::create_user(&connection, 25);
    tip::get_user(&connection, 25);

    let map = artifact_lib::Artifact::new();

    router.get(
        "/artifact/decks/deck/:adc",
        move |request: &mut Request| artifact::decode_and_return_cards(request, &map),
        "adc_deck",
    );

    let map2 = artifact_lib::Artifact::new();
    router.get(
        "/artifact/card_sets",
        move |request: &mut Request| artifact::get_data_view(request, &map2),
        "artifact_card_sets",
    );


    let mut mount = Mount::new();
    // Serve the shared JS/CSS at /static
    mount
        .mount("/", router)
        .mount("/static", Static::new(Path::new("static/")));

    // Run the server.
    Iron::new(mount)
        .http(("0.0.0.0", get_server_port()))
        .unwrap();
}
