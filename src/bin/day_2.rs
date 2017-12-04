extern crate aoc;
use aoc::day_2;

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    // TODO(benl): actual error handling would be really good to figure out. this
    //             silently discards errors.
    let checksum: (f64, f64) = stdin.lock()
                                    .lines()
                                    .filter_map(|l| l.ok())
                                    .filter_map(|r| day_2::parse_row(r).ok())
                                    .map(|r| (day_2::spread(&r), day_2::first_div(&r).unwrap_or(0.0)))
                                    .fold((0.0, 0.0), |(ax, ay), (x, y)| (ax + x, ay + y));

    println!("{:?}", checksum);
}
