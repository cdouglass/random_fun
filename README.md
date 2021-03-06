# Random Fun

## Update

An earlier version of this tool erroneously used the [random.org](random.org) sequence generator API rather than the integers API, getting a random permutation of all numbers in the range [0, 255] rather than 16 randomly chosen numbers from that range. This caused the program to use much more of the user's randomness quota than documented.

## Setup

You will need `cargo` and `rustc` installed. You can get them [here](https://www.rust-lang.org/en-US/install.html) using the `rustup` tool.

## Usage

Clone this repository and navigate to this directory in a terminal.

When you run
```
$ cargo run
```
a random 128x128-pixel RGB bitmap file, `hello.ppm`, will be created in your current directory.

## General information

The image is populated by a manually seeded PRNG that uses the ISAAC algorithm. This is *probably* cryptographically secure, but don't bet your life on it. From the `rand` crate [documentation](https://docs.rs/rand/0.3.16/rand/isaac/struct.IsaacRng.html):

> The ISAAC algorithm is generally accepted as suitable for cryptographic purposes, but this implementation has not be verified as such. Prefer a generator like OsRng that defers to the operating system for cases that need high security.

The seed consists of 16 bytes (128 bits of randomness) obtained from the random.org HTTP API.

## Not implemented (yet)
* Any sort of error handling at all - if a request fails, the program will `panic!()`. However, the worst case here is that the program will quit with an unhelpful error message and fail to write the image file. Rust's memory safety guarantees continue to hold.
* Checking [random.org](random.org) quota status. Since this program requests only small quantities of randomness at a time and uses a PRNG to stretch out its supply of entropy, we're not likely to hit the quota with reasonable usage. You would need to run it over 7,000 times in one day to use up your IP address's quota.
* Including user's email address in user agent (good manners, so that if I made a mistake and the program misbehaves, [random.org](random.org) can let you know)
* CLI flags to choose output file name, image size, or amount of entropy to require
