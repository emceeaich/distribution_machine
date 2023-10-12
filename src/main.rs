use std::cmp::Ordering;
use std::collections::HashMap;
use rand::Rng;

/*
    This is inspired by watching lots of math tuber videos about 
    probability distributions and the central limit theorem.

    It models, empirically, what the distribution of P(x) where x is
    the number of 'guesses' it takes to find a integer distributed
    uniformly between 1 and 100.

    The package does this by running BOUND searches.

    In each iteration we do a binary search for the number.
*/


/*
    The main loop, each pass through it is one trial
*/
fn main() {
    let mut distribution = HashMap::new();
    const BOUND: u32 = 100000000;
    let mut i: u32 = 0;
    loop {
        let tries = guess_number(1, 100);
        i += 1;
        let count_for_tries = distribution.entry(tries).or_insert(0); // make sure each bin is inited with 0
        *count_for_tries += 1; // this appears to be pointer magic

        if i % 10000000 == 0 {
            println!("{} runs", i);
        }

        if i >= BOUND {
            break;
        }
    }

    println!("---------------");
    println!("# guesses : count");
    // print the results as a vertical histogram

    let mut sorted: Vec<_> = distribution.iter().collect();
    sorted.sort_by_key(|a| a.0); // ooh, a rubyism, also a.0 is the key

    for (key, val) in sorted.iter()  {

        let mut bar: String = "X".to_string();
        let size = **val/1000000;
        for _i in 1..size {
            bar += "X";
        }
        println!("{}: {} | {}", key, val, bar);
    }
}

/*
    Perform one trial: pick an number between 1 and 100, then search for it
*/
fn guess_number(low: u32, high: u32) -> u32 {
    const MAX_GUESSES: u32 = 100;

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses: u32 = 0; // this is a loop counter and has to be mutable
    let mut _high = high;
    let mut _low = low;
    // println!("Guessing {}!", secret_number);

    loop {

        let guess: u32 = make_guess(_low, _high);
        guesses += 1;

        // check our 'guess' and adjust the bounds of the search accordingly
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                // println!("Guess {}: {} is too small.", guesses, guess);
                _low = guess + 1;
            },
            Ordering::Greater => {
                // println!("Guess {}: {} is too big.", guesses, guess);
                _high = guess - 1;
            },
            Ordering::Equal => {
                // println!("{} is correct! I took {} guesses.", guess, guesses);
                break;
            } 
        }

        // this was here for debugging the search code, the binary search should be bounded
        // if not, then there's a problem in how the next search value was computed
        if guesses > MAX_GUESSES {
            println!("Something went terribly wrong here. Secret number is: {}, guess is: {}, low: {}, high: {}",
                secret_number, guess, _low, _high);
            break;
        }
    }

    return guesses;
}

/*
    Given an upper and lower bound, compute the next candidate 'guess' in the search
*/
fn make_guess(low: u32, high: u32) -> u32 {
    let guess: f64 = ((high as f64 - low as f64)/2.0 + low as f64).ceil();
    // println!("low: {}, high: {}, guess: {}", low, high, guess);
    return guess as u32;
}
