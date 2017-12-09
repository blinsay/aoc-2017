use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let jumps: Result<Vec<i64>, _> = stdin.lock()
                                          .lines()
                                          .map(Result::unwrap)
                                          .map(|s| s.parse())
                                          .collect();

    if let Ok(jumps) = jumps {
        if jumps.len() == 0 {
            println!("no jumps");
            return;
        }

        println!("regular jumps: {}", JumpCounter::count(|o| o + 1, jumps.clone()));
        println!("strange jumps: {}", JumpCounter::count(|o| if o >=3 { o - 1 } else { o + 1 }, jumps.clone()));
    } else {
        println!("error: invalid jump list")
    }
}


struct JumpCounter<T>
    where T: Fn(i64) -> i64
{
    jumps: Vec<i64>,
    position: Option<usize>,
    offset_fn: T,
}

impl<T> JumpCounter<T>
    where T: Fn(i64) -> i64
{
    pub fn count(offset_fn: T, jumps: Vec<i64>) -> usize {
        if jumps.len() == 0 {
            panic!("empty vec passed to JumpCounter::count")
        }

        let counter = JumpCounter {
            jumps: jumps,
            position: Some(0),
            offset_fn: offset_fn,
        };
        // NOTE: Iterator::count returns the number of in-bounds jumps, since
        // only in-bounds jumps are Some(usize). this fn also includes the last
        // jump to out-of-bounds, so always add one to the number of jumps.
        counter.count() + 1
    }

    fn jump(&mut self) -> Option<usize> {
        if let Some(pos) = self.position {
            let jump_to = (self.jumps[pos] + (pos as i64)) as usize;

            if jump_to >= self.jumps.len() {
                self.position = None;
            } else {
                self.jumps[pos] = (self.offset_fn)(self.jumps[pos]);
                self.position = Some(jump_to);
            }
        }

        self.position
    }
}

impl<T> Iterator for JumpCounter<T>
    where T: Fn(i64) -> i64
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.jump();
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_counter_count() {
        assert_eq!(5, JumpCounter::count(|o| o + 1, vec![0, 3, 0, 1, -3]));
        assert_eq!(10, JumpCounter::count(|o| if o >=3 { o - 1 } else { o + 1 }, vec![0, 3, 0, 1, -3]));
    }
}
