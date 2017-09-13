extern crate hyper;
extern crate hyper_native_tls;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

extern crate rand;
use rand::{Rng, SeedableRng, StdRng};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "hello.ppm";
    let size = 128;
    let header = format!("P6\n{} {}\n255\n", size, size);

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    // get 16x 8-bit integers, for 128 bits of randomness in total
    let url = "https://www.random.org/sequences/?num=128&min=0&max=255&col=1&format=plain&rnd=new";
    let mut response = client.get(url).send().unwrap();
    let mut seed_string = String::new();
    // fills string with numbers, separated by newlines
    response.read_to_string(&mut seed_string).unwrap();
    let seed = string_to_bytes(seed_string);

    // wants to be seeded with usize
    let mut rng = StdRng::from_seed(seed.as_slice());
    // w * h * 3
    let mut bytes = [0u8; 49152];

    rng.fill_bytes(&mut bytes);
    let mut file = File::create(filename).unwrap();
    file.write_all(header.as_bytes()).unwrap();
    file.write_all(&bytes).unwrap();
}

fn string_to_bytes(s: String) -> Vec<usize> {
    let mut bytes = vec![];

    let mut cur = String::new();
    for c in s.chars() {
        if c.is_numeric() {
            cur.push(c);
        } else {
            let byte = cur.parse::<usize>().unwrap();
            bytes.push(byte);
            cur = String::new();
        }
    }

    bytes
}
