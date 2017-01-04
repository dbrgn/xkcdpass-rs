extern crate docopt;
extern crate rand;
extern crate rustc_serialize;
#[macro_use] extern crate lazy_static;

use rand::distributions::{IndependentSample, Range};
use docopt::Docopt;

const USAGE: &'static str = "
Generate XKCD style passwords.

Usage:
  xkcdpass [-c <count>]

Options:
  -h --help     Show this screen.
  -v --version  Show version.
  -c <count>    Number of words [default: 4].
";

static WORDLIST: &'static str = include_str!("wordlist.txt");
lazy_static! {
    static ref MAX_OFFSET: usize = WORDLIST.trim().chars().filter(|c| *c as u32 == 0x0a).count();
    static ref BETWEEN: Range<usize> = Range::new(0, *MAX_OFFSET);
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_c: usize,
}

fn get_random_word() -> &'static str {
    let mut rng = rand::thread_rng();
    let offset = BETWEEN.ind_sample(&mut rng);
    WORDLIST.lines().nth(offset).expect(&format!("Invalid offset: {}", offset))
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let count = args.flag_c;
    for _ in 0..count-1 {
        print!("{} ", get_random_word());
    }
    println!("{}", get_random_word());
}
