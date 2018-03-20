extern crate rand;
#[macro_use(c)]
extern crate cute;

use rand::Rng;
use rand::distributions::{Sample, Range};
use std::vec::Vec;
use std::io::stdin;

fn main() {
    // Roll and score the dice until told not to.
    let mut between = Range::new(1u16, 7);
    let mut rng = rand::thread_rng();
    let num_dice = 6;

    roll(num_dice, &mut rng, &mut between);

    while cont() {
        roll(num_dice, &mut rng, &mut between);
    }
}

fn cont() -> bool {
    // Check user input for whether to continue.
    let mut s = String::new();

    println!("Continue? (Y/N)");
    stdin().read_line(&mut s).expect("Failed to read line");
    match s.trim().as_ref() {
        "Y" => true,
        "y" => true,
        _ => false
    }
}

fn roll<R: Rng>(num_dice: u16, mut rng: &mut R, mut between: &mut Range<u16>) {
    // Roll and score a set of dice.
    let dice = roll_dice(num_dice, &mut rng, &mut between);
    print_dice(&dice);
    println!("Score = {}", score_dice(&dice));
}

fn print_dice(dice: &Vec<u16>) {
    // Pretty print a set of dice.
    println!("Dice:");
    for d in dice.iter() {
        print!("{} ", d);
    }
    println!("");
}

fn score_dice(dice: &Vec<u16>) -> u16 {
    // Score a set of dice.
    Iterator::sum(dice.iter().map(score_die))
}

fn score_die(die: &u16) -> u16 {
    // Score a single die.
    match *die {
        3 => 2,
        5 => 4,
        _ => 0
    }
}

fn roll_dice<R: Rng>(num_rolls: u16, mut rng: &mut R, between: &mut Range<u16>) -> Vec<u16> {
    // Roll a set of dice using provided Rng and range.
    c![between.sample(&mut rng), for _i in 0..num_rolls]
}

