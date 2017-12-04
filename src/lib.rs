pub mod day_1 {
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
}


pub mod day_2 {
    use std::f64;
    use std::str::FromStr;

    pub fn parse_row<T>(s: String) -> Result<Vec<T>, T::Err>
        where T: FromStr {
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
                    return Some(x / y)
                }
            }
        }
        return None
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
            assert_eq!(Some(3.0), first_div(&[9.0, 4.0, 7.0, 3.0,]));
            assert_eq!(Some(2.0), first_div(&[3.0, 8.0, 6.0, 5.0]));
            // from benl
            assert_eq!(None, first_div(&[]));
            assert_eq!(Some(4.0), first_div(&[5.0, 9.0, 2.0, 8.0, 16.0]));
        }
    }
}
