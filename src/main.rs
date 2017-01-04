extern crate rand;
#[macro_use] extern crate lazy_static;

use rand::distributions::{IndependentSample, Range};

static WORDLIST: &'static str = include_str!("wordlist.txt");
lazy_static! {
    static ref MAX_OFFSET: usize = WORDLIST.trim().chars().filter(|c| *c as u32 == 0x0a).count();
    static ref BETWEEN: Range<usize> = Range::new(0, *MAX_OFFSET);
}

fn get_random_word<'a>() -> &'a str {
    let mut rng = rand::thread_rng();
    let offset = BETWEEN.ind_sample(&mut rng);
    WORDLIST.lines().nth(offset).expect(&format!("Invalid offset: {}", offset))
}

fn main() {
    let count = 5;
    for _ in 0..count-1 {
        print!("{} ", get_random_word());
    }
    println!("{}", get_random_word());
}
