# xkcdpass - XKCD Style Password Generator

[![Crates.io][crates-io-badge]][crates-io]

Generate passwords made up by four (or any other number of) random words from a
word list (7776 common English words).

With the default of 4 words, there are 7776^4 = 3656158440062976 possible word
combinations, which corresponds to about 51 bits of entropy. When using 6
words, the entropy is increased to about 77 bit.

https://xkcd.com/936/

## Installing

Install via cargo...

    $ cargo install xkcdpass

...or build locally.

    $ cargo build --release

## Usage

    $ xkcdpass
    pleased excellence space strain
    $ xkcdpass -c 6
    simplified far shade warranty carmen messages

## Word List

By default, this program uses the "Long" wordlist by EFF:
https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

<!-- Badges -->
[crates-io]: https://crates.io/crates/xkcdpass
[crates-io-badge]: https://img.shields.io/crates/v/xkcdpass.svg
