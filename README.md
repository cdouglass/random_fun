#Random Fun

##Setup

You will need `cargo` and `rustc` installed. You can get them [here](https://www.rust-lang.org/en-US/install.html) using the `rustup` tool.

##Usage

Clone this repository and navigate to this directory in a terminal.

When you run
```
$ cargo run
```
a bitmap file named `hello.ppm` will be created in your current directory.

##General information

The image is populated by a manually seeded PRNG that uses the ISAAC algorithm. This is *probably* cryptographically secure, but don't bet your life on it. From the `rand` crate [documentation](https://docs.rs/rand/0.3.16/rand/isaac/struct.IsaacRng.html):

> The ISAAC algorithm is generally accepted as suitable for cryptographic purposes, but this implementation has not be verified as such. Prefer a generator like OsRng that defers to the operating system for cases that need high security.

The seed consists of 16 bytes (128 bits of randomness) obtained from the random.org HTTP API.

##Not implemented (yet)
* Any sort of error handling at all
* Checking random.org quota status (but as this uses a CSPRNG to use entropy efficiently, we're not likely to hit the quota with reasonable usage)
* Including user's email address in user agent (good manners, so that if I made a mistake and the program misbehaves, random.org can let me know)
* CLI flags to choose file name, image size, or amount of entropy to require
