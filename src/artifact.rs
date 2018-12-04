use iron::{status, IronResult, Request, Response};
use regex::Regex;
use router::Router;

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
    let decoded = base64::decode(&String::from(radc)).unwrap();
    let version = decoded.get(0).unwrap() >> 4;
    println!("{}", version);

    let resp = Response::with((status::Ok, format!("ADC: {:?}", decoded)));
    Ok(resp)
}
