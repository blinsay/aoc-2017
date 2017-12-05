use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let (valid, valid_no_angram): (i64, i64) = stdin.lock()
                                      .lines()
                                      .map(Result::unwrap)
                                      .map(|s| (is_valid_passphrase(&s), is_valid_passphrase_no_angrams(&s)))
                                      .map(|(a, b)| (bool_to_int(a), bool_to_int(b)))
                                      .fold((0, 0), |(ax, ay), (x, y)| (ax + x, ay + y));
    println!("valid passwords: {}", valid);
    println!("valid passwords excluding angrams: {}", valid_no_angram);
}

use std::collections::HashSet;

fn bool_to_int(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

fn is_unique_by_key<F>(s: &str, f: F) -> bool
    where F: Fn(&str) -> String
{
    let mut seen = HashSet::new();
    for x in s.split_whitespace() {
        if !seen.insert(f(x)) {
            return false;
        }
    }
    return true;
}

fn is_valid_passphrase(s: &str) -> bool {
    is_unique_by_key(s, str::to_owned)
}

fn is_valid_passphrase_no_angrams(s: &str) -> bool {
    is_unique_by_key(s, sorted_chars)
}

fn sorted_chars(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_passphrase() {
        // valid
        assert!(is_valid_passphrase("aa bb cc dd ee"));
        assert!(is_valid_passphrase("aa bb cc dd aaa"));

        // invalid
        assert!(!is_valid_passphrase("aa bb cc dd aa"));
    }

    #[test]
    fn test_is_valid_passphrase_no_angrams() {
        // valid
        assert!(is_valid_passphrase_no_angrams("abcde fghij"));
        assert!(is_valid_passphrase_no_angrams("a ab abc abd abf abj"));
        assert!(is_valid_passphrase_no_angrams("iiii oiii ooii oooi oooo"));

        // invalid
        assert!(!is_valid_passphrase_no_angrams("abcde xyz ecdab"));
        assert!(!is_valid_passphrase_no_angrams("oiii ioii iioi iiio"));
    }

    #[test]
    fn test_sorted_chars() {
        assert_eq!("", sorted_chars(""));
        assert_eq!("abcdef", sorted_chars("fedcba"));
        assert_eq!("aaabcd", sorted_chars("bacada"));
    }
}
