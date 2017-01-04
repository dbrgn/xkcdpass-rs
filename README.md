# xkcdpass - XKCD Style Password Generator

Generate passwords made up by four (or any other number of) random words from a
word list (9903 common English words).

With the default of 4 words, there are 9903^4 = 9617608981609281 possible word
combinations, which corresponds to about 53 bits of entropy.

## Building

    $ cargo build --release

## Usage

    $ target/release/xkcdpass -c 6
    fault mae sectors pi terror distances
    $ target/release/xkcdpass -c 6
    simplified far shade warranty carmen messages

## Word List

Word list is `google-10000-english-usa-no-swears.txt` taken from
https://github.com/first20hours/google-10000-english.

## License

MIT OR Apache-2
