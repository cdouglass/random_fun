#Random Fun

##Setup

You will need `cargo` and `rustc` installed. You can get them [here](https://www.rust-lang.org/en-US/install.html) using the `rustup` tool.

##Usage

Clone this repository and navigate to this directory in a terminal.

When you run
```
$ cargo run
```
a bitmap file `hello.ppm` format will be created in your current directory.

##General information

The image is populated by a manually seeded PRNG that uses the ISAAC algorithm. This is *probably* cryptographically secure, but don't bet your life on it. From the `rand` crate [documentation](https://docs.rs/rand/0.3.16/rand/isaac/struct.IsaacRng.html):

> The ISAAC algorithm is generally accepted as suitable for cryptographic purposes, but this implementation has not be verified as such. Prefer a generator like OsRng that defers to the operating system for cases that need high security.
