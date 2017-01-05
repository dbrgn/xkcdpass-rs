extern crate docopt;
extern crate rand;
extern crate rustc_serialize;

use std::{process, io, path};
use std::fs::File;
use std::io::Read;
use rand::distributions::{IndependentSample, Range};
use docopt::Docopt;

const USAGE: &'static str = "
Generate XKCD style passwords.

Usage:
  xkcdpass [-c <count>] [-w <list>...]

Options:
  -h --help                 Show this screen.
  -v --version              Show version.
  -c <count>                Number of words [default: 4].
  -w --use-wordlists <list>
";

static DEFAULT_WORDLIST: &'static str = include_str!("wordlist.txt");

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_c: usize,
    flag_w: Option<Vec<String>>,
}

fn load_wordlist(path: String) -> io::Result<String> {
    let path = path::Path::new(&path);
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn load_wordlists(paths: Vec<String>) -> io::Result<String> {
    let mut size = 0;
    let wordlists = paths.into_iter()
        .map(|p| {
            let list = load_wordlist(p);
            if let Ok(ref s) = list {
                size += s.len();
            }
            list
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut string = String::with_capacity(size);

    for list in wordlists {
        string.push_str(&list);
    }

    Ok(string)
}

fn get_random_word(list: &str, offset: usize) -> &str {
    let mut rng = rand::os::OsRng::new().unwrap_or_else(|e| {
        println!("Could not initialize random number generator: {}", e);
        process::exit(1);
    });
    let between = Range::new(0, offset);
    let offset = between.ind_sample(&mut rng);
    list.lines().nth(offset).expect(&format!("Invalid offset: {}", offset))
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let list = if let Some(lists) = args.flag_w {
        match load_wordlists(lists) {
            Ok(l) => l,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    } else {
        DEFAULT_WORDLIST.to_string()
    };
    let offset = list.trim().chars().filter(|c| *c as u32 == 0x0a).count();

    let count = args.flag_c;
    for _ in 1..count {
        print!("{} ", get_random_word(&list, offset));
    }
    println!("{}", get_random_word(&list, offset));
}
