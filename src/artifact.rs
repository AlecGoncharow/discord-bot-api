use iron::{status, IronResult, Request, Response};
use router::Router;
pub fn decode(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let adc = params.find("adc").unwrap();
    let resp = Response::with((status::Ok, format!("ADC: {}", adc)));
    Ok(resp)
}
