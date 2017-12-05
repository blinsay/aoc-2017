use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    // TODO(benl): actual error handling would be really good to figure out. this
    //             silently discards errors.
    let checksum: (f64, f64) = stdin.lock()
                                    .lines()
                                    .filter_map(|l| l.ok())
                                    .filter_map(|r| parse_row(r).ok())
                                    .map(|r| {
                                        (spread(&r), first_div(&r).unwrap_or(0.0))
                                    })
                                    .fold((0.0, 0.0), |(ax, ay), (x, y)| (ax + x, ay + y));

    println!("{:?}", checksum);
}

use std::f64;
use std::str::FromStr;

pub fn parse_row<T>(s: String) -> Result<Vec<T>, T::Err>
    where T: FromStr
{
    s.split_whitespace()
     .map(str::parse)
     .collect()
}

pub fn spread(xs: &[f64]) -> f64 {
    let (mut min, mut max) = (f64::MAX, f64::MIN);
    for x in xs {
        if *x < min {
            min = *x;
        }
        if *x > max {
            max = *x;
        }
    }
    return max - min;
}

pub fn first_div(xs: &[f64]) -> Option<f64> {
    for x in xs {
        for y in xs {
            if (x != y) && x % y == 0.0 {
                return Some(x / y);
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread() {
        assert_eq!(8.0, spread(&[5.0, 1.0, 9.0, 5.0]));
        assert_eq!(4.0, spread(&[7.0, 5.0, 3.0]));
        assert_eq!(6.0, spread(&[2.0, 4.0, 6.0, 8.0]));
    }

    #[test]
    fn test_first_div() {
        // from aoc
        assert_eq!(Some(4.0), first_div(&[5.0, 9.0, 2.0, 8.0]));
        assert_eq!(Some(3.0), first_div(&[9.0, 4.0, 7.0, 3.0]));
        assert_eq!(Some(2.0), first_div(&[3.0, 8.0, 6.0, 5.0]));
        // from benl
        assert_eq!(None, first_div(&[]));
        assert_eq!(Some(4.0), first_div(&[5.0, 9.0, 2.0, 8.0, 16.0]));
    }
}
