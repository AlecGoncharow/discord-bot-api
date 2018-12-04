use iron::{status, IronResult, Request, Response};
use regex::Regex;
use router::Router;

// refer to deck_decoder.php for reference implementation
// https://github.com/ValveSoftware/ArtifactDeckCode/
pub fn decode(req: &mut Request) -> IronResult<Response> {
    let re = Regex::new(r"^ADC").unwrap();
    let params = req.extensions.get::<Router>().unwrap();
    let adc = params.find("adc").unwrap();
    let mut radc = re.replace_all(adc, "");
    radc = radc
        .chars()
        .map(|x| match x {
            '-' => '/',
            '_' => '=',
            _ => x,
        }).collect();

    println!("{}", radc);
    let adc_string = String::from(radc);
    let decoded = base64::decode(&adc_string).unwrap();
    let deck = parse_deck(adc_string, decoded);
    let json_deck = serde_json::to_value(&deck).unwrap();
    let mut resp = Response::new();
    resp.body = Some(std::boxed::Box::new(json_deck.to_string()));
    resp.status = Some(status::Ok);
    resp.headers = iron::Headers::new();
    resp.headers.set(iron::headers::ContentType::json());
    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug)]
struct Hero {
    id: u32,
    turn: u8,
}
#[derive(Serialize, Deserialize, Debug)]
struct Card {
    id: u32,
    count: u8,
}
#[derive(Serialize, Deserialize, Debug)]
struct Deck {
    heroes: Vec<Hero>,
    cards: Vec<Card>,
    name: String,
}

fn parse_deck(_deck_code: String, deck_bytes: Vec<u8>) -> Deck {
    let total_bytes = deck_bytes.len();
    let mut current_byte_index = 0 as usize;
    let version_and_heroes = deck_bytes.get(0).unwrap();
    current_byte_index += 1;

    let version = deck_bytes.get(0).unwrap() >> 4;

    let _checksum = deck_bytes.get(1).unwrap();
    current_byte_index += 1;

    let total_card_bytes = if version > 1 as u8 {
        current_byte_index += 1;
        total_bytes as u8 - deck_bytes.get(2).unwrap()
    } else {
        total_bytes as u8
    };

    let mut num_heroes = 0;
    read_encoded_u32(
        *version_and_heroes as usize,
        3,
        &deck_bytes,
        &mut current_byte_index,
        total_card_bytes as usize,
        &mut num_heroes,
    );

    let mut heroes = Vec::<Hero>::new();
    let mut prev_card_base = 0;
    for curr_hero in 0..num_heroes {
        let mut hero_turn = 0;
        let mut hero_card_id = 0;
        if !read_serialized_card(
            &deck_bytes,
            &mut current_byte_index,
            total_card_bytes as usize,
            &mut prev_card_base,
            &mut hero_turn,
            &mut hero_card_id,
        ) {
            println!(
                "error reading read_serialized_card, curr_hero: {}",
                curr_hero
            );
            break;
        }
        heroes.push(Hero {
            id: hero_card_id,
            turn: hero_turn,
        });
    }

    let mut cards = Vec::<Card>::new();
    prev_card_base = 0;
    while current_byte_index < total_card_bytes as usize {
        let mut card_count = 0;
        let mut card_id = 0;
        if !read_serialized_card(
            &deck_bytes,
            &mut current_byte_index,
            total_card_bytes as usize,
            &mut prev_card_base,
            &mut card_count,
            &mut card_id,
        ) {
            println!(
                "out of card_bytes, current_byte_index: {}",
                current_byte_index
            );
            break;
        }
        cards.push(Card {
            id: card_id,
            count: card_count,
        });
    }

    let name = if current_byte_index < total_card_bytes as usize {
        String::from("has name")
    } else {
        String::from("")
    };

    Deck {
        heroes,
        cards,
        name,
    }
}

fn read_bits_chunk(
    n_chunk: usize,
    n_bits: usize,
    n_curr_shift: usize,
    n_out_bits: &mut u32,
) -> bool {
    let continue_bit = 1 << n_bits;
    let new_bits = n_chunk & (continue_bit - 1);
    *n_out_bits |= (new_bits << n_curr_shift) as u32;

    n_chunk & continue_bit != 0
}

fn read_encoded_u32(
    base_value: usize,
    base_bits: usize,
    deck_bytes: &Vec<u8>,
    start_index: &mut usize,
    end_index: usize,
    out_value: &mut u32,
) {
    *out_value = 0;
    let mut delta_shift = 0;

    if base_bits == 0 || read_bits_chunk(base_value, base_bits, delta_shift, out_value) {
        delta_shift += base_bits;
        loop {
            if *start_index > end_index {
                break;
            }

            let next_byte = deck_bytes.get(*start_index).unwrap();
            *start_index += 1;
            if !read_bits_chunk(*next_byte as usize, 7, delta_shift, out_value) {
                break;
            }

            delta_shift += 7;
        }
    }
}

fn read_serialized_card(
    deck_bytes: &Vec<u8>,
    start_index: &mut usize,
    end_index: usize,
    prev_card_base: &mut u32,
    out_count: &mut u8,
    out_id: &mut u32,
) -> bool {
    //end of the memory block?
    if *start_index > end_index {
        return false;
    }

    //header contains the count (2 bits), a continue flag, and 5 bits of offset data.
    //If we have 11 for the count bits we have the count
    //encoded after the offset
    let header = deck_bytes.get(*start_index).unwrap();
    *start_index += 1;

    let has_extended_count = (header >> 6) == 0x03 as u8;

    //read in the delta, which has 5 bits in the header, then additional bytes while the value is set
    let mut card_delta = 0;
    read_encoded_u32(
        *header as usize,
        5,
        &deck_bytes,
        start_index,
        end_index,
        &mut card_delta,
    );

    *out_id = *prev_card_base + card_delta;

    //now parse the count if we have an extended count
    match has_extended_count {
        true => {
            read_encoded_u32(
                0,
                0,
                &deck_bytes,
                start_index,
                end_index,
                &mut (*out_count as u32),
            );
        }
        false => {
            //the count is just the upper two bits + 1 (since we don't encode zero)
            *out_count = (header >> 6) + 1 as u8;
        }
    }

    *prev_card_base = *out_id;
    true
}
