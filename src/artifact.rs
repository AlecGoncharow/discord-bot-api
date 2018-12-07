use artifact_serde::de::*;
use artifact_serde::*;
use iron::{status, IronResult, Request, Response};
use router::Router;
use std::collections::HashMap;

pub fn decode_and_return_cards(
    req: &mut Request,
    map: &HashMap<usize, Card>,
) -> IronResult<Response> {
    let deck = get_adc_and_decode(req);

    let heroes = deck.heroes;
    let mut cards = deck.cards;
    let mut ret_heroes = Vec::<HeroCard>::new();
    for hero in &heroes {
        let card = match map.get(&(hero.id as usize)) {
            Some(c) => c.clone(),
            None => continue,
        };
        let refer = card.references.clone();
        for r in refer {
            if r.ref_type == "includes" {
                cards.push(DeserializedCard {
                    id: r.card_id,
                    count: r.count,
                });
            }
        }
        ret_heroes.push(HeroCard {
            card: card,
            turn: hero.turn as usize,
        })
    }

    let mut ret_cards = Vec::<CardCard>::new();
    for card in &cards {
        let real_card = match map.get(&(card.id as usize)) {
            Some(c) => c.clone(),
            None => continue,
        };

        ret_cards.push(CardCard {
            card: real_card,
            count: card.count as usize,
        })
    }

    let ret_deck = Deck {
        name: deck.name,
        heroes: ret_heroes,
        cards: ret_cards,
    };

    let json_deck = serde_json::to_value(&ret_deck).unwrap();
    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(json_deck.to_string()));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

    Ok(resp)
}

pub fn decode_and_return_json(req: &mut Request) -> IronResult<Response> {
    let deck = get_adc_and_decode(req);
    let json_deck = serde_json::to_value(&deck).unwrap();

    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(json_deck.to_string()));
    resp.status = Some(status::Ok);
    resp.headers.set(iron::headers::ContentType::json());

    Ok(resp)
}

fn get_adc_and_decode(req: &mut Request) -> DeserializedDeck {
    let params = req.extensions.get::<Router>().unwrap();
    let adc = params.find("adc").unwrap();
    artifact_serde::de::decode(adc).unwrap()
}
