extern crate rand;
use rand::Rng;
use rand::SeedableRng;
use rand::StdRng;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "hello.ppm";
    let size = 128;
    let header = format!("P6\n{} {}\n255\n", size, size);
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = StdRng::from_seed(seed);
    let mut bytes: Vec<u8> = Vec::with_capacity(size * size);
    rng.fill_bytes(&mut bytes);
    let mut file = File::create(filename).unwrap();
    file.write_all(header.as_bytes()).unwrap();
    file.write_all(&bytes).unwrap();
}
