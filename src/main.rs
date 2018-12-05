extern crate base64;
extern crate iron;
extern crate mount;
extern crate regex;
extern crate router;
extern crate staticfile;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod artifact;

use iron::{status, Iron, IronResult, Request, Response};
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::env;
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
    router.get(
        "/artifact/decks/deck/:adc",
        artifact::decode_and_return_cards,
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
