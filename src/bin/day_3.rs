use std::collections::HashMap;

fn main() {
    let mut neighboring_sums: HashMap<(i64, i64), i64> = HashMap::new();
    neighboring_sums.insert((0, 0), 1);

    for (n, (x, y)) in SquareSpiral::new().take(347992).enumerate() {
        let n = n + 1;
        let mut sum = 0;
        for i in x-1..x+2 {
            for j in y-1..y+2 {
                if let Some(v) = neighboring_sums.get(&(i, j)) {
                    sum += v;
                }
            }
        }
        neighboring_sums.insert((x, y), sum);

        // part one
        // if n == 347991 {
        //     println!("n={} ({}, {}) d={} val={}", n, x, y, manhattan_distance(0, 0, x, y), sum);
        // }

        // part two
        if sum >= 347991 {
            println!("n={} ({}, {}) d={} val={}", n, x, y, manhattan_distance(0, 0, x, y), sum);
            return
        }
    }
}

// Generate (x, y) coordinates for a point on the square spiral starting at
// (0, 0), moving to (1, 0) and then proceeding counter-clockwise.
//
//         17 - 16 - 15 - 14 - 13
//         |                |
//         18   5 - 4 - 3   12
//         |    |       |   |
//         19   6   1 - 2   11
//         |    |           |
//         20   7 - 8 - 9 - 10
//         |
//         21 - 22 - 23---> ...
//
// SquareSpiral is an infite iterator.
struct SquareSpiral {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    pos_in_side: usize,
    side_len: usize,
    last_side_len: usize,
}

impl SquareSpiral {
    fn new() -> SquareSpiral {
        SquareSpiral {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
            pos_in_side: 0,
            side_len: 1,
            last_side_len: 0,
        }
    }
}

impl Iterator for SquareSpiral {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        // save the current (x, y) to return
        let (x, y) = (self.x, self.y);

        self.x += self.dx;
        self.y += self.dy;
        self.pos_in_side += 1;

        if self.pos_in_side == self.side_len {
            // reset side
            self.pos_in_side = 0;

            // rotate 90
            let (new_dx, new_dy) = (-1 * self.dy , self.dx);
            self.dx = new_dx;
            self.dy = new_dy;

            //increment side len
            if self.side_len == self.last_side_len {
                self.side_len += 1
            } else {
                self.last_side_len = self.side_len
            }
        }

        Some((x, y))
    }
}


fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
   (x2 - x1).abs() + (y2 - y1).abs()
}
