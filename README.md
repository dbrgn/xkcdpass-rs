# xkcdpass - XKCD Style Password Generator

[![Crates.io][crates-io-badge]][crates-io]

Generate passwords made up by four (or any other number of) random words from a
word list (9903 common English words).

With the default of 4 words, there are 9903^4 = 9617608981609281 possible word
combinations, which corresponds to about 53 bits of entropy.

## Installing

Install via cargo...

    $ cargo install xkcdpass

...or build locally.

    $ cargo build --release

## Usage

    $ xkcdpass -c 6
    fault mae sectors pi terror distances
    $ xkcdpass -c 6
    simplified far shade warranty carmen messages

## Word List

Word list is `google-10000-english-usa-no-swears.txt` taken from
https://github.com/first20hours/google-10000-english.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

<!-- Badges -->
[crates-io]: https://crates.io/crates/xkcdpass
[crates-io-badge]: https://img.shields.io/crates/v/xkcdpass.svg
