use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let digits: Vec<i64> = input.split("").flat_map(|c| c.parse::<i64>()).collect();
    println!("consecutive sum: {}", sum_consecutive_pairs(&digits));
    println!("   circular sum: {}", sum_circular_pairs(&digits));
}

pub fn sum_consecutive_pairs(xs: &[i64]) -> i64 {
    sum_offset_pairs(xs, 1)
}

pub fn sum_circular_pairs(xs: &[i64]) -> i64 {
    sum_offset_pairs(xs, xs.len() / 2)
}

fn sum_offset_pairs(xs: &[i64], offset: usize) -> i64 {
    let mut sum = 0;
    for (i, x) in xs.iter().enumerate() {
        let other = xs[(i + offset) % xs.len()];
        if *x == other {
            sum += x;
        }
    }
    return sum;
}

    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_consecutive_pairs() {
        assert_eq!(3, sum_consecutive_pairs(&[1, 1, 2, 2]));
        assert_eq!(4, sum_consecutive_pairs(&[1, 1, 1, 1]));
        assert_eq!(0, sum_consecutive_pairs(&[1, 2, 3, 4]));
        assert_eq!(9, sum_consecutive_pairs(&[9, 1, 2, 1, 2, 1, 2, 9]));
    }

    #[test]
    fn test_sum_circular_pairs() {
        assert_eq!(6, sum_circular_pairs(&[1, 2, 1, 2]));
        assert_eq!(0, sum_circular_pairs(&[1, 2, 2, 1]));
        assert_eq!(4, sum_circular_pairs(&[1, 2, 3, 4, 2, 5]));
        assert_eq!(12, sum_circular_pairs(&[1, 2, 3, 1, 2, 3]));
        assert_eq!(4, sum_circular_pairs(&[1, 2, 1, 3, 1, 4, 1, 5]));
    }
}
