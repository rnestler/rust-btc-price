extern crate hyper;
extern crate json;

use hyper::Client;
use std::io::Read;

fn main() {
    let client = Client::new();

    let mut current_price_request =
        client.get("https://api.coindesk.com/v1/bpi/currentprice.json").send().unwrap();

    let mut current_price_json = String::new();
    current_price_request.read_to_string(&mut current_price_json).unwrap();

    let parsed = json::parse(&current_price_json).unwrap();

    if let json::JsonValue::Short(price) = parsed["bpi"]["USD"]["rate"] {
        println!("1 BTC = {} USD", price);
    }
}
