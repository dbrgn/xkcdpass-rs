extern crate docopt;
extern crate rand;
#[macro_use]
extern crate serde_derive;

use std::{process, io, path};
use std::fs::File;
use std::io::Read;
use rand::distributions::{IndependentSample, Range};
use docopt::Docopt;

const NAME: &'static str = "xkcdpass";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Generate XKCD style passwords.

Usage:
  xkcdpass [options] [-c <count>] [-w <list>...]

Options:
  -h --help                  Show this screen.
  -v --version               Show version.
  -c --count <count>         Number of words [default: 4].
  -w --use-wordlists <list>  Wordlist to use.
";

static DEFAULT_WORDLIST: &'static str = include_str!("wordlist.txt");

#[derive(Debug, Deserialize)]
struct Args {
    flag_c: usize,
    flag_w: Option<Vec<String>>,
    flag_v: bool,
}

fn load_wordlist(path: String) -> io::Result<String> {
    let path = path::Path::new(&path);
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn load_wordlists(paths: Vec<String>) -> io::Result<Vec<String>> {
    paths.into_iter()
        .map(|p| load_wordlist(p))
        .collect::<Result<Vec<_>, _>>()
}

fn get_random_word<'a>(lines: &[&'a str], max_offset: usize) -> &'a str {
    let mut rng = rand::os::OsRng::new().unwrap_or_else(|e| {
        println!("Could not initialize random number generator: {}", e);
        process::exit(1);
    });
    let between = Range::new(0, max_offset + 1);
    let offset = between.ind_sample(&mut rng);
    lines.get(offset).expect(&format!("Invalid offset: {}", offset))
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Show version and exit
    if args.flag_v {
        println!("{} v{}", NAME, VERSION);
        process::exit(0);
    }

    // Validate count
    if args.flag_c < 1 {
        println!("Error: Word count must be greater than 0");
        process::exit(1);
    }

    // Load wordlist(s)
    let lists = if let Some(lists) = args.flag_w {
        load_wordlists(lists).unwrap_or_else(|e| {
            println!("{}", e);
            process::exit(1);
        })
    } else {
        vec![DEFAULT_WORDLIST.to_string()]
    };
    let lines = lists.iter().flat_map(|l| l.lines()).collect::<Vec<_>>();

    // Get offset
    let max_offset = lists.iter()
        .map(|l| {
            l.trim()
                .chars()
                .filter(|c| *c as u32 == 0x0a)
                .count()
        })
        .sum();

    // Print words
    let count = args.flag_c;
    for _ in 1..count {
        print!("{} ", get_random_word(&lines, max_offset));
    }
    println!("{}", get_random_word(&lines, max_offset));
}
