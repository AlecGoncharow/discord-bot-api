extern crate base64;
extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
//#[macro_use]
extern crate artifact_serde;
extern crate serde_derive;
extern crate serde_json;
mod artifact;

use iron::{status, Iron, IronResult, Request, Response};
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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

fn set_up_deck_map() -> HashMap<usize, artifact_serde::Card> {
    let mut card_file_0 =
        File::open(Path::new("./static/card_set_0.json")).expect("file not found");
    let mut card_file_1 =
        File::open(Path::new("./static/card_set_1.json")).expect("file not found");

    let mut card_set_0 = String::new();
    let mut card_set_1 = String::new();

    card_file_0.read_to_string(&mut card_set_0).unwrap();
    card_file_1.read_to_string(&mut card_set_1).unwrap();

    let sets = vec![card_set_0.as_str(), card_set_1.as_str()];
    artifact_serde::de::map_card_ids_to_cards(sets).unwrap()
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

    let map = set_up_deck_map();
    router.get(
        "/artifact/decks/deck/:adc",
        move |request: &mut Request| artifact::decode_and_return_cards(request, &map),
        "adc_deck",
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
