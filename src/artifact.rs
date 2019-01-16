use iron::{status, IronResult, Request, Response};
use router::Router;

pub fn decode_and_return_cards(
    req: &mut Request,
    map: &artifact_lib::Artifact,
) -> IronResult<Response> {
    let deck = map.get_deck(&get_adc(req));

    let ret_deck = deck.unwrap();

    let json_deck = serde_json::to_value(&ret_deck).unwrap();
    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(json_deck.to_string()));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

    Ok(resp)
}

pub fn decode_and_return_json(req: &mut Request) -> IronResult<Response> {
    let deck = artifact_serde::decode(&get_adc(req)).unwrap();
    let json_deck = serde_json::to_value(&deck).unwrap();

    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(json_deck.to_string()));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

    Ok(resp)
}

pub fn get_data_view(req: &mut Request,
                     map: &artifact_lib::Artifact
) -> IronResult<Response> {
    let mut resp = Response::new();
    let card_sets = serde_json::to_value(&map.card_sets).unwrap();
    resp.body = Some(std::boxed::Box::new(card_sets.to_string()));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

    Ok(resp)
}

fn get_adc(req: &mut Request) -> String {
    let params = req.extensions.get::<Router>().unwrap();
    params.find("adc").unwrap().to_string()
}
