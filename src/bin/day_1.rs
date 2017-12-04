extern crate aoc;
use aoc::day_1::*;

use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let digits: Vec<i64> = input.split("").flat_map(|c| c.parse::<i64>()).collect();
    println!("consecutive sum: {}", sum_consecutive_pairs(&digits));
    println!("   circular sum: {}", sum_circular_pairs(&digits));
}

